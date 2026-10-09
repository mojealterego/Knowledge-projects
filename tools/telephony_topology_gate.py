"""Offline private eSIM/SIP topology check, not an actual telecom provisioner."""
from __future__ import annotations
from dataclasses import dataclass
from ipaddress import ip_network, ip_address
import re

@dataclass(frozen=True)
class TopologyDecision:
    deployable: bool
    findings: tuple[str,...]

_TAG = re.compile(r'^.+:[^:@\s]+$')

def review_topology(plan: dict) -> TopologyDecision:
    if not isinstance(plan,dict):raise TypeError('plan must be a dict')
    problems=[]
    if plan.get('network_owner_approved') is not True:
        problems.append('network_owner_approval_missing')
    if plan.get('operator_spectrum_authorization_verified') is not True:
        problems.append('radio_spectrum_authorization_missing')
    if plan.get('public_number_allocation_claimed') is True:
        problems.append('public_numbers_cannot_be_self_assigned')
    if plan.get('pstn_enabled') is True:
        if plan.get('licensed_sip_trunk_contract_verified') is not True:
            problems.append('pstn_interconnect_not_licensed')
        if plan.get('emergency_call_handling_reviewed') is not True:
            problems.append('emergency_calling_not_reviewed')
    if plan.get('identity_by_ip_only') is not False:
        problems.append('source_ip_is_not_subscriber_identity')
    if plan.get('sip_identity_auth_configured') is not True:
        problems.append('sip_identity_auth_missing')
    if plan.get('exposes_privileged_containers') is not False:
        problems.append('privileged_container_forbidden')
    if plan.get('host_network_for_all_services') is not False:
        problems.append('unbounded_host_network_forbidden')
    if plan.get('secrets_in_plaintext') is not False:
        problems.append('plaintext_secrets_forbidden')
    for img in plan.get('container_images',[]):
        if not isinstance(img,str) or not _TAG.fullmatch(img) or img.endswith(':latest'):
            problems.append('container_image_unpinned')
            break
    if not plan.get('container_images'):
        problems.append('container_images_not_declared')
    if plan.get('subscriber_count',0) not in range(1,1001):
        problems.append('invalid_subscriber_count')
    try:
        net=ip_network(plan['subscriber_subnet'],strict=True)
        hosts=plan.get('subscriber_ips',[])
        if not isinstance(hosts,list) or len(set(hosts))!=len(hosts) or len(hosts)>1000:
            problems.append('duplicate_or_invalid_subscriber_list')
        elif any(ip_address(ip) not in net or ip_address(ip) in (net.network_address,net.broadcast_address) for ip in hosts):
            problems.append('subscriber_ip_outside_usable_range')
        if len(hosts)!=plan.get('subscriber_count'):
            problems.append('subscriber_count_mismatch')
    except (KeyError,ValueError,TypeError):
        problems.append('invalid_subscriber_subnet')
    if plan.get('actual_device_or_radio_tested') is not True:
        problems.append('no_verified_device_integration')
    return TopologyDecision(not problems,tuple(dict.fromkeys(problems)))
