const abi = require("./abi/bridge.json");
const addresses = require('./address.json');
const thorify = require("thorify").thorify;
const Web3 = require("web3");
const web3 = thorify(new Web3(), "http://localhost:8669");

const anchor = require("@project-serum/anchor");
const assert = require("assert");
const { PublicKey } = require("@solana/web3.js");
const { SystemProgram } = anchor.web3;
const TokenInstructions = require("@project-serum/serum").TokenInstructions;

const TOKEN_PROGRAM_ID = new anchor.web3.PublicKey(
  TokenInstructions.TOKEN_PROGRAM_ID.toString()
);

const exo_bridge_address = addresses.bridge_exo;
const gcred_bridge_addres = addresses.bridge_gcred;

const provider = "https://anchor.projectserum.com";
anchor.setProvider(provider);
const gcred_program = anchor.workspace.GcredToken;
const brige_program = anchor.workspace.Bridge;
const exo_program = anchor.workspace.ExoToken;

const exo_token = "";
const gcred_token = "";
const authority = "";


// Contract instance
const exo_bridge_contract = new web3.eth.Contract(abi.abi,exo_bridge_address);
const gcred_bridge_contract = new web3.eth.Contract(abi.abi, gcred_bridge_addres);


gcred_bridge_contract.events.Transfer(), async(error, result) => {
    if(error){
        console.log(error);
    }else{
        if(result.step=="Mint") {
            await brige_program.rpc.proxyBridgeMint(result.amount,{
                accounts:{
                  authority: new PublicKey(authority),
                  mint:new PublicKey(gcred_token),
                  to: new PublicKey(result.to),
                  baseAccount: brige_program.baseAccount.publicKey,
                  tokenProgram: TokenInstructions.TOKEN_PROGRAM_ID,
                  exoTokenProgram: exo_program.programId,
                  exoTokenProgramBaseAccount:exo_program.baseAccount.publicKey,
                  gcredTokenProgram: gcred_program.programId,
                  gcredTokenProgramBaseAccount: gcred_program.baseAccount.publicKey
                }
              })
        } else {
            await brige_program.rpc.proxyBridgeBurn(result.amount, {
                accounts: {
                  authority: new PublicKey(authority),
                  mint:new PublicKey(gcred_token),
                  to: new PublicKey(result.to),
                  baseAccount: brige_program.baseAccount.publicKey,
                  tokenProgram: TokenInstructions.TOKEN_PROGRAM_ID,
                  exoTokenProgram: exo_program.programId,
                  exoTokenProgramBaseAccount:exo_program.baseAccount.publicKey,
                  gcredTokenProgram: gcred_program.programId,
                  gcredTokenProgramBaseAccount: gcred_program.baseAccount.publicKey
                },
              });
        }
    }
};

exo_bridge_contract.events.Transfer(), async(error, result) => {
    if(error){
        console.log(error);
    }else{
        if(result.step=="Mint") {
            await exo_program.rpc.proxyBridgeMint(result.amount,{
                accounts:{
                  authority: new PublicKey(authority),
                  mint:new PublicKey(exo_token),
                  to: new PublicKey(result.to),
                  baseAccount: exo_program.baseAccount.publicKey,
                  tokenProgram: TokenInstructions.TOKEN_PROGRAM_ID,
                  exoTokenProgram: exo_program.programId,
                  exoTokenProgramBaseAccount:exo_program.baseAccount.publicKey,
                  gcredTokenProgram: gcred_program.programId,
                  gcredTokenProgramBaseAccount: gcred_program.baseAccount.publicKey
                }
              })
        } else {
            await exo_program.rpc.proxyBridgeBurn(result.amount, {
                accounts: {
                  authority: new PublicKey(authority),
                  mint:new PublicKey(exo_token),
                  to: new PublicKey(result.to),
                  baseAccount: exo_program.baseAccount.publicKey,
                  tokenProgram: TokenInstructions.TOKEN_PROGRAM_ID,
                  exoTokenProgram: exo_program.programId,
                  exoTokenProgramBaseAccount:exo_program.baseAccount.publicKey,
                  gcredTokenProgram: gcred_program.programId,
                  gcredTokenProgramBaseAccount: gcred_program.baseAccount.publicKey
                },
              });
        }
    }
};