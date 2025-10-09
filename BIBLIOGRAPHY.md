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
- [Trade Appendix - FIX Trading Community](https://www.fixtrading.org/online-specification/trade-appendix/) - Official SideCrossOrdModGrp specification

### Infrastructure Messages
- [Business Area: Infrastructure - FIX Trading Community](https://www.fixtrading.org/online-specification/business-area-infrastructure/) - Official specification for Application Sequencing, Business Message Rejects, Network Status Communication, and User Management messages

### Pre-Trade Messages
- [Business Area: Pre-Trade - FIX Trading Community](https://www.fixtrading.org/online-specification/business-area-pretrade/) - Official specification for Indication, Securities Reference Data, Event Communication, Market Data, and other pre-trade message categories

#### Indication Category
- [IOI (6) - FIX 5.0 SP2](https://www.onixs.biz/fix-dictionary/5.0.sp2/msgType_6_6.html) - Indication of Interest message specification
- [IOI (6) - FIX 5.0](https://www.onixs.biz/fix-dictionary/5.0/msgtype_6_6.html) - Indication of Interest message specification
- [IOI (6) - FIX 4.4](https://www.onixs.biz/fix-dictionary/4.4/msgtype_6_6.html) - Indication of Interest message specification (FIX 4.4)
- [Advertisement (7) - FIX 5.0 SP2](https://www.onixs.biz/fix-dictionary/5.0.sp2/msgType_7_7.html) - Advertisement message specification
- [Advertisement (7) - FIX 4.2](https://www.onixs.biz/fix-dictionary/4.2/msgtype_7_7.html) - Advertisement message specification
- [CrossRequest (DS) - FIX 5.0 SP2 EP266](https://www.onixs.biz/fix-dictionary/5.0.sp2.ep266/msgType_DS_6883.html) - CrossRequest message specification
- [CrossRequest (DS) - FIX Latest](https://fiximate.fixtrading.org/en/FIX.Latest/msg155.html) - CrossRequest message from FIXimate
- [CrossRequest (DS) - FIX 5.0 SP2 EP Dictionary](https://btobits.com/fixopaedia/fixdic50-sp2-ep/message_CrossRequest_DS.html) - CrossRequest message specification
- [CrossRequestAck (DT)](https://fiximate.fixtrading.org/en/FIX.Latest/msg156.html) - CrossRequestAck message from FIXimate
- [FIX.5.0SP2 Message Summary sorted by Type](https://fiximate.fixtrading.org/legacy/en/FIX.5.0SP2/messages_sorted_by_type.html) - Complete message type listing
