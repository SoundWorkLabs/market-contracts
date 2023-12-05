import * as anchor from "@coral-xyz/anchor";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { BN } from "bn.js";
import {
    assetManager,
    biddingDataAcc,
    bidProgram,
    connection,
    findUserEscrowWallet,
    listingDataAcc,
    listProgram,
    nftMint,
    signerOneATA,
    signerOneKp,
    signerTwoATA,
    signerTwoKp,
    vaultTokenAccount,
} from "./config";

describe("SOUNDWORK LIST", async () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    // it("create listing!", async () => {
    //     let tx = await listProgram.methods
    //         .listNft(new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL))
    //         .accounts({
    //             authority: signerOneKp.publicKey,
    //             authorityTokenAccount: signerOneATA,
    //             mint: nftMint,
    //             assetManager,
    //             vaultTokenAccount,
    //             listingData: listingDataAcc,
    //             tokenProgram: TOKEN_PROGRAM_ID,
    //             associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
    //             systemProgram: anchor.web3.SystemProgram.programId,
    //         })
    //     .rpc();

    //     console.log(
    //         `list tx: https://explorer.solana.com/tx/${tx}?cluster=devnet`
    //     );
    // });


    // it("edit listing!", async () => {
    //     let tx = await listProgram.methods
    //         .editListing(new anchor.BN(200 * anchor.web3.LAMPORTS_PER_SOL))
    //         .accounts({
    //             authority: signerOneKp.publicKey,
    //             authorityTokenAccount: signerOneATA,
    //             mint: nftMint,
    //             assetManager,
    //             vaultTokenAccount,
    //             listingData: listingDataAcc,
    //             tokenProgram: TOKEN_PROGRAM_ID,
    //             systemProgram: anchor.web3.SystemProgram.programId,
    //         })
    //         .rpc();

    //     console.log(
    //         `edit listing tx: https://explorer.solana.com/tx/${tx}?cluster=devnet`
    //     );
    // });

    it("delete listing!", async () => {
        let tx = await listProgram.methods
            .deleteListing()
            .accounts({
                authority: signerOneKp.publicKey,
                authorityTokenAccount: signerOneATA,
                mint: nftMint,
                assetManager,
                vaultTokenAccount,
                listingData: listingDataAcc,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();

        console.log(
            `delete listing tx: https://explorer.solana.com/tx/${tx}?cluster=devnet`
        );
    });

    // it("buy listing!", async () => {
    // 	let tx = await listProgram.methods
    // 		.buyListing()
    // 		.accounts({
    // 			buyer: signerTwoKp.publicKey, // ! change
    //             escrowWalletAsBuyer: signerTwoKp.publicKey, 
    // 			ogOwner: signerOneKp.publicKey, // ! change
    // 			buyerTokenAccount: signerTwoATA, // ! change
    // 			mint: nftMint,
    // 			assetManager,
    // 			vaultTokenAccount,
    // 			listingData: listingDataAcc,
    // 			tokenProgram: TOKEN_PROGRAM_ID,
    // associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    // 			systemProgram: anchor.web3.SystemProgram.programId,
    // 		})
    // 		.instruction()

    // 	console.log(
    // 		`buy listing tx: https://explorer.solana.com/tx/${tx}?cluster=devnet`
    // 	);
    // });


    // ------------------------------- ESCROW TESTS -----------------


    // it("deposits sol into escrow!", async () => {
    //     let tx = await listProgram.methods.depositSol(new BN(5 * anchor.web3.LAMPORTS_PER_SOL)).accounts({
    //         owner: signerOneKp.publicKey,
    //         solEscrowWallet: findUserEscrowWallet(signerOneKp.publicKey),
    //         systemProgram: anchor.web3.SystemProgram.programId,
    //     }).rpc();

    //     console.log(
    //         `deposit sol tx: https://explorer.solana.com/tx/${tx}?cluster=devnet`
    //     );
    // });

    // it("withdraws sol from the escrow!", async () => {
    //     let tx = await listProgram.methods.withdrawSol(new BN(1 * anchor.web3.LAMPORTS_PER_SOL)).accounts({
    //         owner: signerOneKp.publicKey,
    //         solEscrowWallet: findUserEscrowWallet(signerOneKp.publicKey),
    //         systemProgram: anchor.web3.SystemProgram.programId,
    //     }).rpc()

    //     console.log(
    //         `withdraw sol tx: https://explorer.solana.com/tx/${tx}?cluster=devnet`
    //     );
    // });
});
