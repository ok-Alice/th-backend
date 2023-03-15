# okAlice Toyota Hachkathon Backend

## Build

    $ cd contracts
    $ sh build-all.sh


## Deploy 

Have a local node running, tested on [Swanky node](https://github.com/AstarNetwork/swanky-node/releases/)

    $ python3 contratcs/deploy.py
    ✅ Deployed assignment/assignment as Assigment for Hash : 5GjhGhEYLkuJFZTwdTMFZ4GpYhHSvyVDFZ3sXkTnPnmvMrHP CodeHash: 0x9916bd157b9ae219e08ad0a63e1e668903f7639fadf67b4c187493e4b113a29b
    ✅ Deployed employee/employee as Employee for Hash : 5Dhgwd2agw49LUUkwmEasyGY5kQd3BEYwWzN1cBWgZyzoAWR CodeHash: 0xdfe4b0e185d389c134d4d908278189517a328417478f8f9905f896fdbee5813d
    ✅ Deployed project as Project : 5HkocnDLcDRPTybQ2HXuA7cKQh6XihUzvaZCfXW6Rinb2BY1 CodeHash: 0x4ed51248db5265f4a53f5b00b455c2e0d765dbab2d1b8e2261b36f31c37c3602

For some unknown reason, the *project* contract is non-functional when deploying through the python script. It has to be deployed manually through the polkadot UI.  Use the contract hashes from the deploy script as argument for the constructor of project
![image](https://user-images.githubusercontent.com/18469570/225447629-b6e046b9-aa8a-417a-902b-cd19f58960a5.png)

Use the address of the deployed contract as an argument for *prepenv.py*

    $ python3 prepenv.py 5HLsbyoZ77HMwCoCMDUFEBdgvQ8T9u2qSRoxNbJa4CnWu3gL
    Alice signs:  <Keypair (address=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY)>
    ✅ Successful transfer from <Keypair (address=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY)> to 5HLsbyoZ77HMwCoCMDUFEBdgvQ8T9u2qSRoxNbJa4CnWu3gL for 100000000000000000
    ✅ Loaded project from 5HLsbyoZ77HMwCoCMDUFEBdgvQ8T9u2qSRoxNbJa4CnWu3gL  CodeHash: 0x4ed51248db5265f4a53f5b00b455c2e0d765dbab2d1b8e2261b36f31c37c3602
    ✅ Loaded employee/employee from 5GjwnvVFm2HsAuAWTwVh81MzqqqBWKWkjxHKQJUMTqZ9Wqz7  CodeHash: 0xdfe4b0e185d389c134d4d908278189517a328417478f8f9905f896fdbee5813d
    ✅ Successful transfer from <Keypair (address=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY)> to 5GjwnvVFm2HsAuAWTwVh81MzqqqBWKWkjxHKQJUMTqZ9Wqz7 for 100000000000000000
    ✅ Loaded assignment/assignment from 5EqSN8RNariZBg83NdGSn5LM7FHdKbwLYKeg8Zq9qBny6cPC  CodeHash: 0x9916bd157b9ae219e08ad0a63e1e668903f7639fadf67b4c187493e4b113a29b
    ✅ Successful transfer from <Keypair (address=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY)> to 5EqSN8RNariZBg83NdGSn5LM7FHdKbwLYKeg8Zq9qBny6cPC for 100000000000000000
      😎 Call Create Project create_project : Events
    ✅ Loaded assignment/assignment from 5FLqbDEvRhAR7q7hnmgTWB36THUb8uWKg7ttUrp85Jq1x94p  CodeHash: 0x9916bd157b9ae219e08ad0a63e1e668903f7639fadf67b4c187493e4b113a29b
    ✅ Successful transfer from <Keypair (address=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY)> to 5FLqbDEvRhAR7q7hnmgTWB36THUb8uWKg7ttUrp85Jq1x94p for 100000000000000000
      😎 Call Mint Employee for alice Minting::mint : Events {Transfer : from:None to:5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY id:{'U64': 1} }
      😎 Call Employee metadata alice Minting::assign_metadata : Events
