#!/usr/bin/env python3
"""
Script to split the large types.rs file into smaller, categorized modules.
"""

import re
import os
from pathlib import Path

# Define module categories and their enum patterns
CATEGORIES = {
    'order': [
        'Side', 'OrdType', 'OrdStatus', 'ExecType', 'DKReason', 'ExecAckStatus',
    ],
    'cross': [
        'CrossType', 'CrossPrioritization',
    ],
    'program_trading': [
        'BidType', 'ProgRptReqs', 'ListExecInstType', 'ListStatusType',
        'ListOrderStatus', 'BidDescriptorType', 'SideValueInd', 'NetGrossInd',
        'PriceType', 'OrderEntryAction',
    ],
    'mass_operations': [
        'MassCancelRequestType', 'MassCancelResponse', 'MassActionType',
        'MassActionResponse', 'MassStatusReqType', 'OrderResponseLevel',
    ],
    'multileg': [
        'MultilegReportingType', 'MultilegModel', 'MultilegPriceMethod',
    ],
    'infrastructure': [
        'BusinessRejectReason', 'NetworkRequestType', 'NetworkStatusResponseType',
        'NetworkSystemStatus', 'ApplReqType', 'ApplResponseType', 'ApplReportType',
        'UserRequestType', 'UserStatus',
    ],
    'indication': [
        'IOITransType', 'IOIQltyInd', 'AdvSide', 'AdvTransType', 'QtyType',
    ],
    'communication': [
        'EmailType', 'NewsRefType', 'NewsCategory',
    ],
    'quotation': [
        'QuoteType', 'QuoteRequestType', 'QuoteCancelType', 'QuoteResponseLevel',
        'QuoteRequestRejectReason', 'QuoteStatus', 'QuoteCondition',
    ],
    'market_data': [
        'MDReqRejReason', 'MDUpdateType', 'SubscriptionRequestType', 'MDEntryType',
    ],
    'market_structure': [
        'TradSesStatus', 'TradSesMethod', 'TradSesMode', 'TradSesStatusRejReason',
        'TradSesUpdateAction', 'MarketUpdateAction',
    ],
    'securities': [
        'SecurityRequestType', 'SecurityRequestResult', 'SecurityListRequestType',
        'SecurityUpdateAction', 'SecurityTradingStatus', 'HaltReason',
    ],
    'post_trade': [
        # Allocation
        'AllocTransType', 'AllocType', 'AllocStatus', 'AllocRejCode',
        'AllocCancReplaceReason', 'AllocIntermedReqType', 'AllocReportType',
        'AvgPxIndicator', 'AllocRequestStatus', 'MatchStatus', 'IndividualAllocRejCode',
        # Confirmation
        'ConfirmType', 'ConfirmStatus', 'ConfirmTransType', 'AffirmStatus',
        'ConfirmRejReason',
        # Position
        'PosReqType', 'PosTransType', 'PosMaintAction', 'PosMaintResult',
        'PosReqStatus', 'PosReqResult', 'PosType', 'PosQtyStatus',
        'SettlPriceType', 'AdjustmentType', 'PosAmtType',
        # Settlement
        'SettlInstMode', 'SettlInstTransType', 'SettlInstSource', 'StandInstDbType',
        'SettlInstReqRejCode', 'SettlObligMode',
        # Trade Capture
        'TradeRequestType', 'TradeRequestResult', 'TradeRequestStatus',
        'TradeReportType', 'TrdType', 'TrdSubType', 'MatchType',
    ],
}

def find_enum_category(enum_name):
    """Find which category an enum belongs to."""
    for category, enums in CATEGORIES.items():
        if enum_name in enums:
            return category
    # Default to 'other' if not categorized
    return 'other'

def parse_types_file(file_path):
    """Parse types.rs and extract enum blocks with their implementations."""
    with open(file_path, 'r') as f:
        content = f.read()

    # Split into blocks using a simpler approach
    # Find all enum declarations and extract the block including docs and impl
    blocks = []
    lines = content.split('\n')
    i = 0

    while i < len(lines):
        line = lines[i]

        # Look for enum declarations
        if 'pub enum ' in line:
            match = re.search(r'pub enum (\w+)', line)
            if match:
                enum_name = match.group(1)

                # Go back to capture doc comments and derives
                start_idx = i
                while start_idx > 0 and (lines[start_idx - 1].strip().startswith('///') or
                                         lines[start_idx - 1].strip().startswith('#[') or
                                         lines[start_idx - 1].strip() == ''):
                    start_idx -= 1
                    if lines[start_idx].strip() and not lines[start_idx].strip().startswith(('///', '#[')):
                        start_idx += 1
                        break

                # Find end of enum
                enum_end = i
                brace_count = 0
                for j in range(i, len(lines)):
                    brace_count += lines[j].count('{')
                    brace_count -= lines[j].count('}')
                    if brace_count == 0 and '}' in lines[j]:
                        enum_end = j
                        break

                # Look for impl block
                impl_end = enum_end
                for j in range(enum_end + 1, min(enum_end + 200, len(lines))):
                    if f'impl {enum_name}' in lines[j]:
                        # Found impl, find its end
                        brace_count = 0
                        for k in range(j, len(lines)):
                            brace_count += lines[k].count('{')
                            brace_count -= lines[k].count('}')
                            if brace_count == 0 and '}' in lines[k]:
                                impl_end = k
                                break
                        break
                    elif 'pub enum ' in lines[j] or 'pub struct ' in lines[j]:
                        # Hit next enum/struct, no impl
                        break

                # Extract the block
                block_content = '\n'.join(lines[start_idx:impl_end + 1])
                blocks.append({
                    'enum_name': enum_name,
                    'content': block_content,
                    'category': find_enum_category(enum_name)
                })

                i = impl_end + 1
                continue

        i += 1

    return blocks

def write_module_files(blocks, output_dir):
    """Write blocks to their respective module files."""
    # Group blocks by category
    categorized = {}
    for block in blocks:
        category = block['category']
        if category not in categorized:
            categorized[category] = []
        categorized[category].append(block['content'])

    # Create output directory
    os.makedirs(output_dir, exist_ok=True)

    # Write each category to a file
    for category, contents in categorized.items():
        file_path = os.path.join(output_dir, f'{category}.rs')
        with open(file_path, 'w') as f:
            f.write('use serde::{Deserialize, Serialize};\n\n')
            f.write('\n\n'.join(contents))
            f.write('\n')
        print(f"Created {file_path} with {len(contents)} enums")

    return list(categorized.keys())

def create_mod_file(categories, output_dir):
    """Create mod.rs that re-exports all types."""
    mod_path = os.path.join(output_dir, 'mod.rs')
    with open(mod_path, 'w') as f:
        f.write('// Re-export all types from sub-modules\n\n')

        # Declare modules
        for category in sorted(categories):
            f.write(f'mod {category};\n')

        f.write('\n// Re-export all public types\n')
        for category in sorted(categories):
            f.write(f'pub use {category}::*;\n')

    print(f"Created {mod_path}")

def main():
    source_file = 'src/types.rs'
    output_dir = 'src/types'

    print(f"Parsing {source_file}...")
    blocks = parse_types_file(source_file)
    print(f"Found {len(blocks)} enum blocks")

    print(f"\nWriting module files to {output_dir}/...")
    categories = write_module_files(blocks, output_dir)

    print(f"\nCreating mod.rs...")
    create_mod_file(categories, output_dir)

    print(f"\n✅ Successfully split types.rs into {len(categories)} module files!")
    print(f"\nNext steps:")
    print(f"1. Rename src/types.rs to src/types.rs.bak")
    print(f"2. Test compilation with: cargo build --jobs 1")
    print(f"3. Test everything with: cargo test --jobs 1")

if __name__ == '__main__':
    main()
