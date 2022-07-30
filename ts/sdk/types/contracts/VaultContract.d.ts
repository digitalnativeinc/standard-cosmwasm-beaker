/**
* This file was automatically generated by cosmwasm-typescript-gen@0.3.9.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the cosmwasm-typescript-gen generate command to regenerate this file.
*/
import { CosmWasmClient, ExecuteResult, SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
export declare type ExecuteMsg = {
    liquidate: {
        [k: string]: unknown;
    };
} | {
    withdraw_collateral: {
        amount: Uint128;
        [k: string]: unknown;
    };
} | {
    deposit_collateral: {
        [k: string]: unknown;
    };
} | {
    borrow_more: {
        amount: Uint128;
        [k: string]: unknown;
    };
} | {
    paydebt: {
        amount: Uint128;
        [k: string]: unknown;
    };
} | {
    close_vault: {
        [k: string]: unknown;
    };
};
export declare type Uint128 = string;
export interface GetBalancesResponse {
    c: Coin;
    d: Coin;
    [k: string]: unknown;
}
export interface Coin {
    amount: Uint128;
    denom: string;
    [k: string]: unknown;
}
export interface GetStateResponse {
    borrow: Uint128;
    collateral: string;
    debt: string;
    last_updated: number;
    manager: string;
    sfr: number;
    v1: string;
    vault_id: number;
    [k: string]: unknown;
}
export interface InstantiateMsg {
    borrow: Uint128;
    collateral: string;
    created_at: number;
    debt: string;
    manager: string;
    v1: string;
    vault_id: number;
    [k: string]: unknown;
}
export declare type QueryMsg = {
    get_state: {
        [k: string]: unknown;
    };
} | {
    get_balances: {
        [k: string]: unknown;
    };
};
export interface State {
    borrow: Uint128;
    collateral: string;
    debt: string;
    ex_sfr: number;
    last_updated: number;
    manager: string;
    v1: string;
    vault_id: number;
    [k: string]: unknown;
}
export interface VaultReadOnlyInterface {
    contractAddress: string;
    getState: () => Promise<GetStateResponse>;
    getBalances: () => Promise<GetBalancesResponse>;
}
export declare class VaultQueryClient implements VaultReadOnlyInterface {
    client: CosmWasmClient;
    contractAddress: string;
    constructor(client: CosmWasmClient, contractAddress: string);
    getState: () => Promise<GetStateResponse>;
    getBalances: () => Promise<GetBalancesResponse>;
}
export interface VaultInterface extends VaultReadOnlyInterface {
    contractAddress: string;
    sender: string;
    liquidate: (fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
    withdrawCollateral: ({ amount }: {
        amount: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
    depositCollateral: (fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
    borrowMore: ({ amount }: {
        amount: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
    paydebt: ({ amount }: {
        amount: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
    closeVault: (fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
}
export declare class VaultClient extends VaultQueryClient implements VaultInterface {
    client: SigningCosmWasmClient;
    sender: string;
    contractAddress: string;
    constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string);
    liquidate: (fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
    withdrawCollateral: ({ amount }: {
        amount: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
    depositCollateral: (fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
    borrowMore: ({ amount }: {
        amount: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
    paydebt: ({ amount }: {
        amount: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
    closeVault: (fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
}
//# sourceMappingURL=VaultContract.d.ts.map