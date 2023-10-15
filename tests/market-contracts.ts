import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import {
	getAssociatedTokenAddressSync,
	getAccount,
	TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
// import { mplTokenMetadata } from '@metaplex-foundation/mpl-token-metadata';

import { MarketContracts } from "../target/types/market_contracts";

describe("market-contracts", async () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const authority = anchor.AnchorProvider.env().wallet as anchor.Wallet;

	const program = anchor.workspace
		.MarketContracts as Program<MarketContracts>;

	let nftMint = new anchor.web3.PublicKey(
		"7HnPDNsroSKVzcLq23Ub2uMcGRda9zEUGppj3WYK3Nry"
	);

	const [assetManager] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from("soundwork")],
		program.programId
	);
	const [listingDataAcc] = anchor.web3.PublicKey.findProgramAddressSync(
		[nftMint.toBuffer(), Buffer.from("ryo")],
		program.programId
	);

	let userTokenAccount = getAssociatedTokenAddressSync(
		nftMint,
		authority.publicKey
	); // ? we know it exists
	// let vaultTokenAccount = getAssociatedTokenAddressSync(nftMint, assetManager, true); // ! check this
	let [vaultTokenAccount] = anchor.web3.PublicKey.findProgramAddressSync(
		[assetManager.toBuffer()],
		program.programId
	); // ! check this

	console.log("vault token account    -> ", vaultTokenAccount.toBase58());
	console.log("asset manager account  -> ", assetManager.toBase58());

	it("create listing!", async () => {
		let tx = await program.methods
			.listNft(200)
			.accounts({
				authority: authority.publicKey,
				authorityTokenAccount: userTokenAccount,
				mint: nftMint,
				assetManager,
				vaultTokenAccount,
				listingData: listingDataAcc,
				tokenProgram: TOKEN_PROGRAM_ID,
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.rpc();

		console.log(
			`list tx: https://explorer.solana.com/tx/${tx}?cluster=devnet`
		);
	});

	//
});
