# Bibliography

This file tracks reference materials consulted during the development of fixie.

## FIX Protocol Specifications

### Message Type Specifications (OnixS)
- [CrossOrderCancelReplaceRequest (t)](https://www.onixs.biz/fix-dictionary/5.0.sp2/msgType_t_116.html) - Cross order modification message
- [CrossOrderCancelRequest (u)](https://www.onixs.biz/fix-dictionary/5.0.sp2/msgType_u_117.html) - Cross order cancellation message
- [NewOrderCross (s)](https://www.onixs.biz/fix-dictionary/5.0.sp2/msgType_s_115.html) - Cross order submission message

### Cross Order Field Specifications (OnixS)
- [CrossType (549)](https://www.onixs.biz/fix-dictionary/5.0.sp2/tagnum_549.html) - Type of cross order (AON, IOC, OneSide, SamePrice)
- [CrossPrioritization (550)](https://www.onixs.biz/fix-dictionary/5.0.sp2/tagnum_550.html) - Cross order prioritization (None, BuySide, SellSide)

### Repeating Groups
- [Repeating groups in FIX Protocol - JavaRevisited](https://javarevisited.blogspot.com/2011/02/repeating-groups-in-fix-protcol.html) - Overview and examples of repeating group format
- [FIX Repeating Group - OnixS](https://ref.onixs.biz/fix-repeating-group.html) - Technical specification
- [OnixS C++ FIX Engine: FIX Repeating Groups](https://ref.onixs.biz/cpp-fix-engine-guide/group__fix-protocol-repeating-group.html) - Implementation guide
- [Where to get information to correctly parse repeating groups in FIX? - Stack Overflow](https://stackoverflow.com/questions/30449300/where-to-get-information-to-correctly-parse-repeating-groups-in-fix) - Community discussion
