# cgraph


## types
- Graph
- Node
- Edge
- Descriptor
- Attribute

## Traits
The following traits should be usable across all major types

- `root()`
- `graphof()` 
- `contains(obj)`
- 'delete(obj)`

# Notes

## Disciplines
Ability to let client apps manage 
resources.  Goal to avoid this being 
necessary;  perhaps implement via 
allowed Traits 

| Discipline   | Description              | C ident      | Implemented |
|--------------|--------------------------|--------------|-------------| 
| ID Namespace | client control of id     | `AGDISC_ID`  | no          |  
| Memory       | client control of memory | `AGDISC_MEM` | no          |
| IO           | client control of IO     | `AGDISC_IO`  | no          |


