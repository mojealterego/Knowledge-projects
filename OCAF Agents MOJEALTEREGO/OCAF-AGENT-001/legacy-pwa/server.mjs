import http from 'node:http';
import { readFile } from 'node:fs/promises';
import { extname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(fileURLToPath(new URL('.', import.meta.url)), 'public');
const port = process.env.PORT || 8787;

const json = (res, status, body) => {
  res.writeHead(status, {'Content-Type':'application/json; charset=utf-8','Cache-Control':'no-store'});
  res.end(JSON.stringify(body));
};
async function body(req) { const chunks=[]; for await (const c of req) chunks.push(c); return JSON.parse(Buffer.concat(chunks).toString('utf8') || '{}'); }

async function openai(body) {
  const key = process.env.OPENAI_API_KEY;
  if (!key) throw new Error('OPENAI_API_KEY is not configured.');
  const r = await fetch('https://api.openai.com/v1/responses', {
    method:'POST', headers:{'Authorization':`Bearer ${key}`,'Content-Type':'application/json'},
    body: JSON.stringify(body)
  });
  const data = await r.json();
  if (!r.ok) throw new Error(data?.error?.message || `OpenAI HTTP ${r.status}`);
  return data;
}

let ffTokenCache = { token:null, exp:0 };
async function adobeToken() {
  const id=process.env.FIREFLY_SERVICES_CLIENT_ID, secret=process.env.FIREFLY_SERVICES_CLIENT_SECRET;
  if (!id || !secret) throw new Error('Adobe Firefly credentials are not configured.');
  if (ffTokenCache.token && Date.now() < ffTokenCache.exp-60000) return ffTokenCache.token;
  const r=await fetch('https://ims-na1.adobelogin.com/ims/token/v3',{method:'POST',headers:{'Content-Type':'application/x-www-form-urlencoded'},body:new URLSearchParams({client_id:id,client_secret:secret,grant_type:'client_credentials',scope:'openid,AdobeID,firefly_api'}).toString()});
  const d=await r.json();
  if(!r.ok) throw new Error(d?.error_description || `Adobe auth HTTP ${r.status}`);
  ffTokenCache={token:d.access_token,exp:Date.now()+(d.expires_in||86399)*1000};
  return d.access_token;
}

async function fireflyGenerate({prompt,aspectRatio='1:1',numVariations=1}) {
  const token=await adobeToken();
  const r=await fetch('https://firefly-api.adobe.io/v3/images/generate',{method:'POST',headers:{'Content-Type':'application/json','Accept':'application/json','x-api-key':process.env.FIREFLY_SERVICES_CLIENT_ID,'Authorization':`Bearer ${token}`},body:JSON.stringify({prompt,aspectRatio,numVariations,modelId:'firefly_image'})});
  const d=await r.json();
  if(!r.ok) throw new Error(d?.error?.message || d?.message || `Firefly HTTP ${r.status}`);
  return d;
}

async function fireflyUpload(buffer, contentType) {
  const token=await adobeToken();
  const r=await fetch('https://firefly-api.adobe.io/v2/storage/image',{method:'POST',headers:{'Authorization':`Bearer ${token}`,'X-Api-Key':process.env.FIREFLY_SERVICES_CLIENT_ID,'Content-Type':contentType,'Content-Length':String(buffer.length)},body:buffer});
  const d=await r.json(); if(!r.ok) throw new Error(d?.message||`Upload HTTP ${r.status}`); return d;
}

const routes = {
  '/api/health': async ()=>({ok:true, openai:Boolean(process.env.OPENAI_API_KEY), adobe:Boolean(process.env.FIREFLY_SERVICES_CLIENT_ID&&process.env.FIREFLY_SERVICES_CLIENT_SECRET)}),
  '/api/analyze': async (b)=>openai({model:process.env.OPENAI_MODEL||'gpt-5.6-luna',input:[{role:'system',content:'You are a mobile virtual art director for professional photography. Analyze the supplied image and return a concise, actionable retouching plan in JSON: subject, composition, lighting, skin/texture, color, background, risks, recommended workflow. Preserve identity and pose unless explicitly requested otherwise.'},{role:'user',content:[{type:'input_text',text:b.instruction||'Analyze this photograph for professional retouching.'},{type:'input_image',image_url:b.image}]}],text:{format:{type:'json_schema',name:'photo_analysis',schema:{type:'object',additionalProperties:false,properties:{subject:{type:'string'},composition:{type:'string'},lighting:{type:'string'},skin_texture:{type:'string'},color:{type:'string'},background:{type:'string'},risks:{type:'array',items:{type:'string'}},workflow:{type:'array',items:{type:'string'}}},required:['subject','composition','lighting','skin_texture','color','background','risks','workflow']}}}}),
  '/api/chat': async (b)=>openai({model:process.env.OPENAI_MODEL||'gpt-5.6-luna',input:[{role:'system',content:'You are WDA, a virtual art director focused on photographic retouching. Give professional, concrete directions. When useful, produce an English production prompt for Adobe Firefly. Do not claim to have directly controlled Adobe Firefly unless an API action was actually executed.'},{role:'user',content:[{type:'input_text',text:b.message||''},...(b.image?[{type:'input_image',image_url:b.image}]:[])]}]}),
  '/api/firefly/generate': async (b)=>fireflyGenerate(b),
  '/api/firefly/upload': async (b)=>{
    if(!b.base64) throw new Error('Missing base64 image.');
    const m=String(b.base64).match(/^data:([^;]+);base64,(.+)$/); if(!m) throw new Error('Expected data URL.');
    const d=await fireflyUpload(Buffer.from(m[2],'base64'),m[1]); return d;
  }
};

const mime={'.html':'text/html; charset=utf-8','.js':'text/javascript; charset=utf-8','.css':'text/css; charset=utf-8','.json':'application/json; charset=utf-8','.svg':'image/svg+xml'};
const server=http.createServer(async(req,res)=>{
  try {
    if(req.method==='POST' && routes[req.url]) return json(res,200,await routes[req.url](await body(req)));
    if(req.method==='GET' && req.url.startsWith('/api/health')) return json(res,200,await routes['/api/health']({}));
    const path=req.url==='/'?'/index.html':req.url;
    const buf=await readFile(join(root,path)); res.writeHead(200,{'Content-Type':mime[extname(path)]||'application/octet-stream'}); res.end(buf);
  } catch(e) { json(res,500,{error:e.message||String(e)}); }
});
server.listen(port,()=>console.log(`WDA Photo Agent running on http://localhost:${port}`));
