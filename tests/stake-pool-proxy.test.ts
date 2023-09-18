import {startAnchor} from "solana-bankrun";
import {
    Keypair,
    PublicKey,
    Signer,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    Transaction,
    TransactionInstruction
} from "@solana/web3.js";
import {StakePoolProxy, IDL} from "../target/types/stake_pool_proxy";
import {Program} from "@project-serum/anchor";
import {BankrunProvider} from "anchor-bankrun";

test("anchor", async () => {
    const stakePoolProgramId = new PublicKey("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy");
    const stakePoolProxyProgramId = new PublicKey("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
    const LST = new PublicKey("LSTxxxnJzKDFSLr4dUkPcmCf5VyryEqzPLz5j4bpxFp");

    const context = await startAnchor(".", [{name: "stake_pool", programId: stakePoolProgramId}], []);
    const provider = new BankrunProvider(context);

    const program = new Program<StakePoolProxy>(
        IDL,
        stakePoolProxyProgramId,
        provider,
    );

    const client = context.banksClient;
    const payer = context.payer;
    const blockhash = context.lastBlockhash;

    // Ephemeral SOL account just to do the transfer
    const userSolTransfer = new Keypair();
    const signers: Signer[] = [userSolTransfer];
    const instructions: TransactionInstruction[] = [];

    const depositAmount = 999 * 1e9;
    const balanceBefore = await client.getBalance(payer.publicKey);
    console.log("balanceBefore", balanceBefore);

    // Create the ephemeral SOL account
    instructions.push(
        SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: userSolTransfer.publicKey,
            lamports: depositAmount,
        }),
    );

    // Sol deposit by proxy
    instructions.push(
        await program.methods.depositAllSol().accounts({
            lamportsFrom: userSolTransfer.publicKey,
            stakePoolProgram: stakePoolProgramId,
            rent: SYSVAR_RENT_PUBKEY,
            poolMint: LST,
            poolTokensTo: payer.publicKey,
            // TODO
        }).instruction());

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(...instructions);

    tx.sign(payer, ...signers);
    await client.processTransaction(tx);

    const balanceAfter = await client.getBalance(payer.publicKey);
    console.log("balanceAfter", balanceAfter);
});