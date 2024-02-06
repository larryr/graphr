# Notes

Key Elements
- GVC - Context
- GVJ - Jobs
  - output format 
  - output target
- Graph
  - universal space
  - multipl layers
    - pages array
        - page
          - nodes
          - edges
          - clusters
          - html anchors
- Layout Jobs
- Render Jobs

Output can be viewed as a hierarchy of document components. At the 
highest level is the job, representing an output format and target. 
Bound to a job might be multiple graphs, each embedded in some 
universal space. Each graph may be partitioned into multiple layers 
as determined by a graph’s layers attribute, if any. Each layer may 
be divided into a 2-dimensional array of pages. A page will then 
contain nodes, edges, and clusters. Each of these may contain an 
HTML anchor. During rendering, each component is reflected in paired 
calls to its corresponding begin ... and end ... functions. The layer 
and anchor components are omitted if there is only a single layer or 
the enclosing component has no browser information.


## GVC - Context


## GVJ - Jobs


#