// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
// import {
// 	getAssociatedTokenAddressSync,
// 	getAccount,
// 	TOKEN_PROGRAM_ID,
// } from "@solana/spl-token";

// import { SoundworkList } from "../target/types/soundwork_list";
// import { nftMint } from "./config";

// describe("soundwork-list", async () => {
// 	// Configure the client to use the local cluster.
// 	anchor.setProvider(anchor.AnchorProvider.env());

// 	const authority = anchor.AnchorProvider.env().wallet as anchor.Wallet;

// 	const program = anchor.workspace
// 		.MarketContracts as Program<SoundworkList>;

// 	const [assetManager] = anchor.web3.PublicKey.findProgramAddressSync(
// 		[Buffer.from("soundwork")],
// 		program.programId
// 	);
// 	const [listingDataAcc] = anchor.web3.PublicKey.findProgramAddressSync(
// 		[nftMint.toBuffer(), Buffer.from("ryo")],
// 		program.programId
// 	);

// 	let userTokenAccount = getAssociatedTokenAddressSync(
// 		nftMint,
// 		authority.publicKey
// 	); // ? we know it exists
// 	let vaultTokenAccount = getAssociatedTokenAddressSync(nftMint, assetManager, true); // ! check this


// 	it("create listing!", async () => {
// 		let tx = await program.methods
// 			.listNft(new anchor.BN(100 * anchor.web3.LAMPORTS_PER_SOL))
// 			.accounts({
// 				authority: authority.publicKey,
// 				authorityTokenAccount: userTokenAccount,
// 				mint: nftMint,
// 				assetManager,
// 				vaultTokenAccount,
// 				listingData: listingDataAcc,
// 				tokenProgram: TOKEN_PROGRAM_ID,
// 				associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
// 				systemProgram: anchor.web3.SystemProgram.programId,
// 			})
// 			.rpc();

// 		console.log(
// 			`list tx: https://explorer.solana.com/tx/${tx}?cluster=devnet`
// 		);
// 	});

// 	// it("edit listing!", async () => {
// 	// 	let tx = await program.methods
// 	// 		.editListing(new anchor.BN(200 * anchor.web3.LAMPORTS_PER_SOL))
// 	// 		.accounts({
// 	// 			authority: authority.publicKey,
// 	// 			authorityTokenAccount: userTokenAccount,
// 	// 			mint: nftMint,
// 	// 			assetManager,
// 	// 			vaultTokenAccount,
// 	// 			listingData: listingDataAcc,
// 	// 			tokenProgram: TOKEN_PROGRAM_ID,
// 	// 			systemProgram: anchor.web3.SystemProgram.programId,
// 	// 		})
// 	// 		.rpc();

// 		// console.log(
// 		// 	`edit listing tx: https://explorer.solana.com/tx/${tx}?cluster=devnet`
// 		// );
// 	// });

// 	// it("delete listing!", async () => {
// 	// 	let tx = await program.methods
// 	// 		.removeListing()
// 	// 		.accounts({
// 	// 			authority: authority.publicKey,
// 	// 			authorityTokenAccount: userTokenAccount,
// 	// 			mint: nftMint,
// 	// 			assetManager,
// 	// 			vaultTokenAccount,
// 	// 			listingData: listingDataAcc,
// 	// 			tokenProgram: TOKEN_PROGRAM_ID,
// 	// 			systemProgram: anchor.web3.SystemProgram.programId,
// 	// 		})
// 	// 		.rpc();

// 	// 	console.log(
// 	// 		`delete listing tx: https://explorer.solana.com/tx/${tx}?cluster=devnet`
// 	// 	);
// 	// });


// 	// it("buy listing!", async () => {
// 	// 	let tx = await program.methods
// 	// 		.buyListing()
// 	// 		.accounts({
// 	// 			buyer: authority.publicKey, // ! change
// 	// 			ogOwner: authority.publicKey, // ! change
// 	// 			buyerTokenAccount: userTokenAccount, // ! change
// 	// 			mint: nftMint,
// 	// 			assetManager,
// 	// 			vaultTokenAccount,
// 	// 			listingData: listingDataAcc,
// 	// 			tokenProgram: TOKEN_PROGRAM_ID,
// 	// 			systemProgram: anchor.web3.SystemProgram.programId,
// 	// 		})
// 	// 		.rpc();

// 	// 	console.log(
// 	// 		`buy listing tx: https://explorer.solana.com/tx/${tx}?cluster=devnet`
// 	// 	);
// 	// });

// 	//
// });
