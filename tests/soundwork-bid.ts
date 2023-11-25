import * as anchor from "@coral-xyz/anchor";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { BN } from "bn.js";
import {
	assetManager,
	biddingDataAcc,
	bidProgram,
	connection,
	// findBiddingWallet,
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

describe("SOUNDWORK BID", async () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	// let expire_ts = new Date().getTime();
	// it("Creates a bid for an NFT!", async () => {
	// 	let ix = await bidProgram.methods
	// 		.makeBid(
	// 			new BN(1 * anchor.web3.LAMPORTS_PER_SOL),
	// 			new BN(expire_ts)
	// 		)
	// 		.accounts({
	// 			bidder: signerTwoKp.publicKey,
	// 			mint: nftMint,
	// 			biddingDataAcc,
	// 			listingDataAcc,
	// 			solEscrowWallet: findUserEscrowWallet(signerTwoKp.publicKey),
	// 			soundworkList: listProgram.programId,
	// 			systemProgram: anchor.web3.SystemProgram.programId,
	// 		})
	// 		.instruction();

	// 	let tx = new anchor.web3.Transaction().add(ix);

	// 	let txHash = await anchor.web3.sendAndConfirmTransaction(
	// 		connection,
	// 		tx,
	// 		[signerTwoKp]
	// 	);

	// 	console.log(
	// 		`create a bid tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet`
	// 	);
	// });

	// it("accepts bid!", async () => {
	// 	let ix = await bidProgram.methods
	// 		.acceptBid()
	// 		.accounts({
	// 			seller: signerOneKp.publicKey,
	// biddingDataAcc,
	// 			listingDataAcc,
	// 			buyer: signerTwoKp.publicKey,
	// 			mint: nftMint,
	// 			// biddingWallet: findBiddingWallet(signerTwoKp.publicKey),
	// 			buyerSolEscrow: findUserEscrowWallet(signerTwoKp.publicKey),
	// 			buyerTokenAcc: signerTwoATA,
	// 			assetManager,
	// 			vaultTokenAcc: vaultTokenAccount,
	// 			soundworkList: listProgram.programId,
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 			associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
	// 			systemProgram: anchor.web3.SystemProgram.programId,
	// 		})
	// 		.instruction();

	// 	let tx = new anchor.web3.Transaction().add(ix);

	// 	let txHash = await anchor.web3.sendAndConfirmTransaction(
	// 		connection,
	// 		tx,
	// 		[signerOneKp]
	// 	);

	// 	console.log(
	// 		`accept bid tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet`
	// 	);
	// });

	// it("rejects bid!", async () => {
		// let ix = await bidProgram.methods
		// 	.rejectBid()
		// 	.accounts({
		// 		seller: signerOneKp.publicKey,
		// 		listingDataAcc,
		// 		buyer: signerTwoKp.publicKey,
		// 		buyerSolEscrow: findUserEscrowWallet(signerTwoKp.publicKey),
		// 		biddingDataAcc,
		// 		soundworkList: listProgram.programId,
		// 		systemProgram: anchor.web3.SystemProgram.programId,
		// 	})
		// 	.instruction();

	// 	let tx = new anchor.web3.Transaction().add(ix);

	// 	let txHash = await anchor.web3.sendAndConfirmTransaction(
	// 		connection,
	// 		tx,
	// 		[signerOneKp]
	// 	);

	// 	console.log(
	// 		`reject bid tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet`
	// 	);
	// });

	// it("deletes a bid!", async () => {
		// let ix = await bidProgram.methods
		// 	.deleteBid()
		// 	.accounts({
		// 		bidder: signerTwoKp.publicKey,
		// 		biddingDataAcc,
		// 		solEscrowWallet: findUserEscrowWallet(signerTwoKp.publicKey),
		// 		soundworkList: listProgram.programId,
		// 		systemProgram: anchor.web3.SystemProgram.programId,
		// 	})
		// 	.instruction();

	// 	let tx = new anchor.web3.Transaction().add(ix);

	// 	let txHash = await anchor.web3.sendAndConfirmTransaction(
	// 		connection,
	// 		tx,
	// 		[signerTwoKp]
	// 	);

	// 	console.log(
	// 		`delete bid tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet`
	// 	);
	// });
});
