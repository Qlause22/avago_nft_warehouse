# Avago NFT Warehouse

This repository addresses a specific limitation in the Radix component ecosystem, where assets and business logic are treated as separate entities. While this structure offers modularity, it also introduces challenges, especially when flexibility is needed for assets like NFTs.

The primary objective of this project is to address these limitations within Radix’s architecture, enabling NFT holders to list their assets across multiple marketplaces seamlessly. By bridging the gap between asset management and business logic, this solution aims to create a more flexible ecosystem for asset interaction on Radix.

## Solution

To tackle the challenges of asset flexibility on the Radix network, we introduced a two-asset model:

1. **Owner NFT**: Represents any asset the user wants to sell. Users need this asset to create vNFT later.

2. **Vault NFT (vNFT)**: This unique token grants permissions for any designated account or component to withdraw the asset. This NFT is easily recognizable across marketplaces and holds essential information about the asset.

Owner NFT and vNFT have the same address for each other (it will help anyone to recognize the asset). The vNFT is also linked to the Owner NFT.

## Integration

The vNFT features a unique behavior we call **Dynamic Asset Behavior**, which allows the asset's behavior to change at runtime. This advantage enables integrators to modify manifest instructions without needing to integrate at the blueprint level.

### Step-by-Step Integration Process

1. **Open Method**: 
   - Call the `open` method, which can be called only once. This method requires a string argument with the value either `withdraw` or `deposit`.
     - If `withdraw` is chosen, the vNFT behavior will change to allow withdrawals while preventing any deposits. This ensures that if the vNFT is withdrawn, it cannot be transferred to another account; only the user can withdraw it to extract their asset.
     - If `deposit` is chosen, the vNFT behavior will change to allow deposits while preventing any withdrawals. This guarantees that minted vNFTs can be deposited into an account or component.

2. **Proceed**:
   - The `open` method returns a transient token. Users are required to call a second method, either:
     - `mint_vnft` (for minting)
     - `take_vnft_assets` (to withdraw the asset).

3. **Finish Method**:
   - Finally, call the `finish` method, passing the transient token. This action changes the vNFT's behavior to a Soulbond NFT.


