* examples use cases
- as a user, I should be able to request capacity
  - globally via a single number
  - regionally, via a set of numbers
  - zonally, via a set of numbers within a region
- as an operator, I am free to optimize within these constraints
  - i.e. expectation is that workloads will be registered in ALL zones, and I am free to optimize across them
    - spot arbitrage
    - outages
      - for outages, in particular want to be globally aware of splits and have a region proactively scale out based on desired global capacity
      - ex. if 50/50 and region A goes down, region B autoscaler should be doubling capacity
        - therefore, need a controller that considers both higher level ratio and target metric

- a zonally defined service means only zonal load balancing takes place
- regionally defined balances down to zonal balancers(?)
  - can set min/max constraints (burstiness?), but does not actually manage zonal capacity
- global balances down to regional balancers
  - determines based off fixed regional balance
  - recursively feedbacks down into zonal, etc.

* Creation of workloads will be registered zonally and replicated globally via gddb
  - workload is the entity plus global relation?
  - 

* TODO: services should be able to set optional min/max, but have a target metric PID controller

* given configuration has a workload location and source, priority of workload registrations for a given location are
  - source == current, location == _
  - source != current, location != current != source
  - source != current, location == source
* tl;dr i treat local records as gospel, i treat backup sources as more authoritiative (because human generated?), and then treat sources for authorative as normal
  - expectation is that case 1/2 are the outcome of human intervention?
  - q: should we just collapse 1/2 into a case where source is not for location?

* zonal/regional autoscalers have regional/global weights and a known configuration
