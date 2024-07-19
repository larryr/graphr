# Graphr API

The primary intent of Graphr is to provide a graph library compatible with the Dot language.

Therefore, Graphr implements a Dot language parser to create a graph and subsequently to render the
graph in a variety of formats and using a variety of layout algorithms.

# Programmatic API

Graphr provides a programatic API to create and manipulate graphs.  However, this API is not intended
to be a like-graphviz C API. 

However, the following provides some guidance on comparing the Graphviz API with Graphr:

| Graphviz API   | kind     | Graphr API              | Note                             |
|----------------|----------|-------------------------|----------------------------------|    
| `Agraph_t`     | type     | `Graph`                 |                                  |
| `Agnode_t`     | type     | `Node`                  |                                  |
| `Agedge_t`     | type     | `Edge`                  |                                  |
| `Agdesc_t`     | type     | `GraphDesc`             |                                  |
| `Agdisc_t`     | type     |                         |                                  |
| `Agsym_t`      | type     |                         |                                  |
| `Agrec_t`      | type     |                         |                                  |
| `Agcbdisc_t`   | type     |                         |                                  |
| Graphs         |          |                         |                                  |
| `agopen`       | function | `Graph::new()`          |                                  |
| `agread`       | function | `Graph::read()`         | implement std read()             |
| `agwrite`      | function | `Graph::write()`        | implement std write()            |
| `agisdirected` | function | `Graph::is_directed()`  |                                  |
| `agconcat`     | function | `Graph::concat()`       |                                  |
| `agnnodes`     | function | `Graph::node_count()`   |                                  |
| `agclose`      | function |                         |                                  |
| `agmemread`    | function |                         | use read() with a byte io buffer |
| `agreadline`   | function |                         |                                  |
| `agsetfile`    | function |                         |                                  |
| Subgraphs      |          |                         |                                  |
| `agsubg`       | function | `Graph::new_subgraph()` |                                  |
| `agsubg`       | function | `Graph::get_subgraph()` |                                  |
| `agidsubg`     | function | `Graph::new_subgraph()` |                                  |
| `agparent`     | function | `Graph::parent()`       |                                  |
| `agfstsubg`    | function |                         | provide an iterator of subgraph  |
| `agnxtsubg`    | function |                         |                                  |
| `agdelsubg`    | function | `Graph::del_subgraph()` |                                  |