/*!
 * standard-cosmwasm-beaker-sdk v 0.0.1
 * (c) hyungsukkang
 * Released under the MIT OR Apache-2.0 License.
 */

/******************************************************************************
Copyright (c) Microsoft Corporation.

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THIS SOFTWARE.
***************************************************************************** */
/* global Reflect, Promise */

var extendStatics = function(d, b) {
    extendStatics = Object.setPrototypeOf ||
        ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
        function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
    return extendStatics(d, b);
};

function __extends(d, b) {
    if (typeof b !== "function" && b !== null)
        throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
    extendStatics(d, b);
    function __() { this.constructor = d; }
    d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
}

function __awaiter(thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
}

function __generator(thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
}

/**
* This file was automatically generated by cosmwasm-typescript-gen@0.3.9.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the cosmwasm-typescript-gen generate command to regenerate this file.
*/
var VaultQueryClient = /** @class */ (function () {
    function VaultQueryClient(client, contractAddress) {
        var _this = this;
        this.getState = function () { return __awaiter(_this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                        get_state: {}
                    })];
            });
        }); };
        this.getBalances = function () { return __awaiter(_this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                        get_balances: {}
                    })];
            });
        }); };
        this.client = client;
        this.contractAddress = contractAddress;
        this.getState = this.getState.bind(this);
        this.getBalances = this.getBalances.bind(this);
    }
    return VaultQueryClient;
}());
var VaultClient = /** @class */ (function (_super) {
    __extends(VaultClient, _super);
    function VaultClient(client, sender, contractAddress) {
        var _this = _super.call(this, client, contractAddress) || this;
        _this.liquidate = function (fee, memo, funds) {
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    switch (_a.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                liquidate: {}
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _a.sent()];
                    }
                });
            });
        };
        _this.withdrawCollateral = function (_a, fee, memo, funds) {
            var amount = _a.amount;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                withdraw_collateral: {
                                    amount: amount
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.depositCollateral = function (fee, memo, funds) {
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    switch (_a.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                deposit_collateral: {}
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _a.sent()];
                    }
                });
            });
        };
        _this.borrowMore = function (_a, fee, memo, funds) {
            var amount = _a.amount;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                borrow_more: {
                                    amount: amount
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.paydebt = function (_a, fee, memo, funds) {
            var amount = _a.amount;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                paydebt: {
                                    amount: amount
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.closeVault = function (fee, memo, funds) {
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    switch (_a.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                close_vault: {}
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _a.sent()];
                    }
                });
            });
        };
        _this.client = client;
        _this.sender = sender;
        _this.contractAddress = contractAddress;
        _this.liquidate = _this.liquidate.bind(_this);
        _this.withdrawCollateral = _this.withdrawCollateral.bind(_this);
        _this.depositCollateral = _this.depositCollateral.bind(_this);
        _this.borrowMore = _this.borrowMore.bind(_this);
        _this.paydebt = _this.paydebt.bind(_this);
        _this.closeVault = _this.closeVault.bind(_this);
        return _this;
    }
    return VaultClient;
}(VaultQueryClient));

var VaultContract = /*#__PURE__*/Object.freeze({
    __proto__: null,
    VaultQueryClient: VaultQueryClient,
    VaultClient: VaultClient
});

/**
* This file was automatically generated by cosmwasm-typescript-gen@0.3.9.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the cosmwasm-typescript-gen generate command to regenerate this file.
*/
var NftQueryClient = /** @class */ (function () {
    function NftQueryClient(client, contractAddress) {
        var _this = this;
        this.ownerOf = function (_a) {
            var includeExpired = _a.includeExpired, tokenId = _a.tokenId;
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            owner_of: {
                                include_expired: includeExpired,
                                token_id: tokenId
                            }
                        })];
                });
            });
        };
        this.approval = function (_a) {
            var includeExpired = _a.includeExpired, spender = _a.spender, tokenId = _a.tokenId;
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            approval: {
                                include_expired: includeExpired,
                                spender: spender,
                                token_id: tokenId
                            }
                        })];
                });
            });
        };
        this.approvals = function (_a) {
            var includeExpired = _a.includeExpired, tokenId = _a.tokenId;
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            approvals: {
                                include_expired: includeExpired,
                                token_id: tokenId
                            }
                        })];
                });
            });
        };
        this.allOperators = function (_a) {
            var includeExpired = _a.includeExpired, limit = _a.limit, owner = _a.owner, startAfter = _a.startAfter;
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            all_operators: {
                                include_expired: includeExpired,
                                limit: limit,
                                owner: owner,
                                start_after: startAfter
                            }
                        })];
                });
            });
        };
        this.numTokens = function () { return __awaiter(_this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                        num_tokens: {}
                    })];
            });
        }); };
        this.contractInfo = function () { return __awaiter(_this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                        contract_info: {}
                    })];
            });
        }); };
        this.nftInfo = function (_a) {
            var tokenId = _a.tokenId;
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            nft_info: {
                                token_id: tokenId
                            }
                        })];
                });
            });
        };
        this.allNftInfo = function (_a) {
            var includeExpired = _a.includeExpired, tokenId = _a.tokenId;
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            all_nft_info: {
                                include_expired: includeExpired,
                                token_id: tokenId
                            }
                        })];
                });
            });
        };
        this.tokens = function (_a) {
            var limit = _a.limit, owner = _a.owner, startAfter = _a.startAfter;
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            tokens: {
                                limit: limit,
                                owner: owner,
                                start_after: startAfter
                            }
                        })];
                });
            });
        };
        this.allTokens = function (_a) {
            var limit = _a.limit, startAfter = _a.startAfter;
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            all_tokens: {
                                limit: limit,
                                start_after: startAfter
                            }
                        })];
                });
            });
        };
        this.minter = function () { return __awaiter(_this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                        minter: {}
                    })];
            });
        }); };
        this.client = client;
        this.contractAddress = contractAddress;
        this.ownerOf = this.ownerOf.bind(this);
        this.approval = this.approval.bind(this);
        this.approvals = this.approvals.bind(this);
        this.allOperators = this.allOperators.bind(this);
        this.numTokens = this.numTokens.bind(this);
        this.contractInfo = this.contractInfo.bind(this);
        this.nftInfo = this.nftInfo.bind(this);
        this.allNftInfo = this.allNftInfo.bind(this);
        this.tokens = this.tokens.bind(this);
        this.allTokens = this.allTokens.bind(this);
        this.minter = this.minter.bind(this);
    }
    return NftQueryClient;
}());
var NftClient = /** @class */ (function (_super) {
    __extends(NftClient, _super);
    function NftClient(client, sender, contractAddress) {
        var _this = _super.call(this, client, contractAddress) || this;
        _this.transferNft = function (_a, fee, memo, funds) {
            var recipient = _a.recipient, tokenId = _a.tokenId;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                transfer_nft: {
                                    recipient: recipient,
                                    token_id: tokenId
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.sendNft = function (_a, fee, memo, funds) {
            var contract = _a.contract, msg = _a.msg, tokenId = _a.tokenId;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                send_nft: {
                                    contract: contract,
                                    msg: msg,
                                    token_id: tokenId
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.approve = function (_a, fee, memo, funds) {
            var expires = _a.expires, spender = _a.spender, tokenId = _a.tokenId;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                approve: {
                                    expires: expires,
                                    spender: spender,
                                    token_id: tokenId
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.revoke = function (_a, fee, memo, funds) {
            var spender = _a.spender, tokenId = _a.tokenId;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                revoke: {
                                    spender: spender,
                                    token_id: tokenId
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.approveAll = function (_a, fee, memo, funds) {
            var expires = _a.expires, operator = _a.operator;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                approve_all: {
                                    expires: expires,
                                    operator: operator
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.revokeAll = function (_a, fee, memo, funds) {
            var operator = _a.operator;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                revoke_all: {
                                    operator: operator
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.mint = function (_a, fee, memo, funds) {
            var extension = _a.extension, owner = _a.owner, tokenId = _a.tokenId, tokenUri = _a.tokenUri;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                mint: {
                                    extension: extension,
                                    owner: owner,
                                    token_id: tokenId,
                                    token_uri: tokenUri
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.burn = function (_a, fee, memo, funds) {
            var tokenId = _a.tokenId;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                burn: {
                                    token_id: tokenId
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.client = client;
        _this.sender = sender;
        _this.contractAddress = contractAddress;
        _this.transferNft = _this.transferNft.bind(_this);
        _this.sendNft = _this.sendNft.bind(_this);
        _this.approve = _this.approve.bind(_this);
        _this.revoke = _this.revoke.bind(_this);
        _this.approveAll = _this.approveAll.bind(_this);
        _this.revokeAll = _this.revokeAll.bind(_this);
        _this.mint = _this.mint.bind(_this);
        _this.burn = _this.burn.bind(_this);
        return _this;
    }
    return NftClient;
}(NftQueryClient));

var NftContract = /*#__PURE__*/Object.freeze({
    __proto__: null,
    NftQueryClient: NftQueryClient,
    NftClient: NftClient
});

/**
* This file was automatically generated by cosmwasm-typescript-gen@0.3.9.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the cosmwasm-typescript-gen generate command to regenerate this file.
*/
var VaultManagerQueryClient = /** @class */ (function () {
    function VaultManagerQueryClient(client, contractAddress) {
        var _this = this;
        this.getVaultConfig = function (_a) {
            var clt = _a.clt;
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            get_vault_config: {
                                clt: clt
                            }
                        })];
                });
            });
        };
        this.getConfig = function () { return __awaiter(_this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                        get_config: {}
                    })];
            });
        }); };
        this.client = client;
        this.contractAddress = contractAddress;
        this.getVaultConfig = this.getVaultConfig.bind(this);
        this.getConfig = this.getConfig.bind(this);
    }
    return VaultManagerQueryClient;
}());
var VaultManagerClient = /** @class */ (function (_super) {
    __extends(VaultManagerClient, _super);
    function VaultManagerClient(client, sender, contractAddress) {
        var _this = _super.call(this, client, contractAddress) || this;
        _this.initialize = function (_a, fee, memo, funds) {
            var admin = _a.admin, factory = _a.factory, stablecoin = _a.stablecoin, v1 = _a.v1, vaultCodeId = _a.vaultCodeId;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                initialize: {
                                    admin_: admin,
                                    factory_: factory,
                                    stablecoin_: stablecoin,
                                    v1_: v1,
                                    vault_code_id_: vaultCodeId
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.createVault = function (_a, fee, memo, funds) {
            var dAmount = _a.dAmount;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                create_vault: {
                                    d_amount: dAmount
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.setVaultConfig = function (_a, fee, memo, funds) {
            var cDecimal = _a.cDecimal, clt = _a.clt, lfr = _a.lfr, mcr = _a.mcr, poolId = _a.poolId, sfr = _a.sfr;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                set_vault_config: {
                                    c_decimal_: cDecimal,
                                    clt: clt,
                                    lfr_: lfr,
                                    mcr_: mcr,
                                    pool_id_: poolId,
                                    sfr_: sfr
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.client = client;
        _this.sender = sender;
        _this.contractAddress = contractAddress;
        _this.initialize = _this.initialize.bind(_this);
        _this.createVault = _this.createVault.bind(_this);
        _this.setVaultConfig = _this.setVaultConfig.bind(_this);
        return _this;
    }
    return VaultManagerClient;
}(VaultManagerQueryClient));

var VaultManagerContract = /*#__PURE__*/Object.freeze({
    __proto__: null,
    VaultManagerQueryClient: VaultManagerQueryClient,
    VaultManagerClient: VaultManagerClient
});

/**
* This file was automatically generated by cosmwasm-typescript-gen@0.3.9.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the cosmwasm-typescript-gen generate command to regenerate this file.
*/
var TokenfactoryQueryClient = /** @class */ (function () {
    function TokenfactoryQueryClient(client, contractAddress) {
        var _this = this;
        this.getDenom = function (_a) {
            var creatorAddress = _a.creatorAddress, subdenom = _a.subdenom;
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            get_denom: {
                                creator_address: creatorAddress,
                                subdenom: subdenom
                            }
                        })];
                });
            });
        };
        this.client = client;
        this.contractAddress = contractAddress;
        this.getDenom = this.getDenom.bind(this);
    }
    return TokenfactoryQueryClient;
}());
var TokenfactoryClient = /** @class */ (function (_super) {
    __extends(TokenfactoryClient, _super);
    function TokenfactoryClient(client, sender, contractAddress) {
        var _this = _super.call(this, client, contractAddress) || this;
        _this.createDenom = function (_a, fee, memo, funds) {
            var subdenom = _a.subdenom;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                create_denom: {
                                    subdenom: subdenom
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.changeAdmin = function (_a, fee, memo, funds) {
            var denom = _a.denom, newAdminAddress = _a.newAdminAddress;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                change_admin: {
                                    denom: denom,
                                    new_admin_address: newAdminAddress
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.mintTokens = function (_a, fee, memo, funds) {
            var amount = _a.amount, denom = _a.denom, mintToAddress = _a.mintToAddress;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                mint_tokens: {
                                    amount: amount,
                                    denom: denom,
                                    mint_to_address: mintToAddress
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.burnTokens = function (_a, fee, memo, funds) {
            var amount = _a.amount, burnFromAddress = _a.burnFromAddress, denom = _a.denom;
            if (fee === void 0) { fee = "auto"; }
            return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                burn_tokens: {
                                    amount: amount,
                                    burn_from_address: burnFromAddress,
                                    denom: denom
                                }
                            }, fee, memo, funds)];
                        case 1: return [2 /*return*/, _b.sent()];
                    }
                });
            });
        };
        _this.client = client;
        _this.sender = sender;
        _this.contractAddress = contractAddress;
        _this.createDenom = _this.createDenom.bind(_this);
        _this.changeAdmin = _this.changeAdmin.bind(_this);
        _this.mintTokens = _this.mintTokens.bind(_this);
        _this.burnTokens = _this.burnTokens.bind(_this);
        return _this;
    }
    return TokenfactoryClient;
}(TokenfactoryQueryClient));

var TokenfactoryContract = /*#__PURE__*/Object.freeze({
    __proto__: null,
    TokenfactoryQueryClient: TokenfactoryQueryClient,
    TokenfactoryClient: TokenfactoryClient
});

export { NftContract, TokenfactoryContract, VaultContract, VaultManagerContract };
//# sourceMappingURL=index.mjs.map
