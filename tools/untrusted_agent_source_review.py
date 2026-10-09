"""Non-executing static triage of untrusted agent text.

No imports of supplied code, network, shell, or scanning capabilities.
Individual signals are heuristics requiring independent human review.
"""
from __future__ import annotations
from dataclasses import dataclass
from pathlib import Path
import ast
import re

RULES = {
  'unbounded_shell': r'(?:create_subprocess_shell|subprocess\.Popen|os\.system\s*\(|shell\s*=\s*True)',
  'active_network_recon': r'\bnmap\b|\bmasscan\b|\bss7\b|\bhlr[\s_-]*lookup',
  'intrusion_intent': r'\bzero[ -]click\b|\bzero[ -]day\b|\bexploitat|\bsteal credentials\b|\bcredential theft\b|\bpassword cracking\b',
  'covert_surveillance': r'\binwigil|\bcovert tracking\b|\bstalking\b|\btrack target\b',
  'unbounded_autonomy': r'\bnieskończon\w* pęt\w*|\binfinite loop\b|\bwhile\s+(?:True|self\.running)\s*:',
  'safety_bypass': r'\bbypass (?:security|firewall|waf)\b|\bno safety barriers\b|\bbrak jakichkolwiek barier\b',
  'unsafe_default_allow': r'(?s)def\s+(?:validate_input|verify_boundary)\s*\([^)]*\)[^\n]*:\s*(?:#.*\n\s*)*return\s+True',
}
@dataclass(frozen=True)
class Review:
    safe_to_execute_automatically: bool
    signal_names: tuple[str, ...]
    syntax_parseable_as_python: bool | None
    inspection_only: bool = True

def inspect_text(text: str, *, python_source: bool = False) -> Review:
    if not isinstance(text,str):raise TypeError('text must be str')
    if len(text)>1_000_000:raise ValueError('text length limit exceeded')
    signals = [name for name,pattern in RULES.items() if re.search(pattern,text,re.IGNORECASE)]
    parses=None
    if python_source:
        try:
            ast.parse(text)
            parses=True
        except SyntaxError:
            parses=False
            signals.append('invalid_python_syntax')
    return Review(False,tuple(signals),parses)

if __name__ == '__main__':
    import argparse,json
    parser=argparse.ArgumentParser(description='Review source text ONLY; no code execution')
    parser.add_argument('source',type=Path)
    parser.add_argument('--python',action='store_true')
    args=parser.parse_args()
    if args.source.stat().st_size>1_000_000:
        parser.error('file too large')
    result=inspect_text(args.source.read_text(encoding='utf-8'),python_source=args.python)
    print(json.dumps(result.__dict__,indent=2,ensure_ascii=False))
