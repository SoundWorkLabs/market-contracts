import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { homedir } from "os";
import { readFileSync } from "fs";
import { Program, Wallet } from "@coral-xyz/anchor";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";

import { SoundworkBid } from "../target/types/soundwork_bid";
import { SoundworkList } from "../target/types/soundwork_list";

export const authority = anchor.AnchorProvider.env().wallet as Wallet;
export const connection = new anchor.web3.Connection(
	anchor.web3.clusterApiUrl("devnet")
);
export const provider = new anchor.AnchorProvider(
	connection,
	authority,
	anchor.AnchorProvider.defaultOptions()
);

const bidProgramId = new PublicKey(
	"CVuoapcC1RG8y1m86eChkXmynPi4FaykDC8jM8soMZ4j"
);
const bidProgramIdl = JSON.parse(
	readFileSync(
		"/home/jimii/Documents/crew/market-contracts/target/idl/soundwork_bid.json",
		"utf8"
	)
);
export const bidProgram = new Program(
	bidProgramIdl,
	bidProgramId,
	provider
) as Program<SoundworkBid>;

const listProgramId = new PublicKey(
	"EUmBNHvFqhkA6Uaqv6pDup5ESHKCqoAweQ4kzAMjNZhX"
);
const listProgramIdl = JSON.parse(
	readFileSync(
		"/home/jimii/Documents/crew/market-contracts/target/idl/soundwork_list.json",
		"utf8"
	)
);
export const listProgram = new Program(
	listProgramIdl,
	listProgramId,
	provider
) as Program<SoundworkList>;

const KEYPAIR_PATH_ONE = homedir() + "/.config/solana/id.json";
export const signerOneKp = Keypair.fromSecretKey(
	Buffer.from(JSON.parse(readFileSync(KEYPAIR_PATH_ONE, "utf-8")))
);
export const borrower = new Wallet(signerOneKp);

const KEYPAIR_PATH_TWO = homedir() + "/.config/solana/id-new.json";
export const signerTwoKp = Keypair.fromSecretKey(
	Buffer.from(JSON.parse(readFileSync(KEYPAIR_PATH_TWO, "utf-8")))
);

export const nftMint = new PublicKey(
	"4KhcMPEvxqoW9J7fBbZn88Q5ksg9wsihjfoPnkZEJ5Vv"
);

/// derive PDAs
// ! list program
export const [assetManager] = PublicKey.findProgramAddressSync(
	[Buffer.from("soundwork")],
	listProgram.programId
);
export const [listingDataAcc] = PublicKey.findProgramAddressSync(
	[nftMint.toBuffer(), Buffer.from("ryo")],
	listProgram.programId
);
export function findUserEscrowWallet(user: PublicKey): PublicKey {
	const [userEscrowWaller] = PublicKey.findProgramAddressSync(
		[user.toBuffer(), Buffer.from("Hitori")],
		listProgram.programId
	);
	return userEscrowWaller;
}

// ! bid program
export const [biddingDataAcc] = PublicKey.findProgramAddressSync(
	[nftMint.toBuffer(), Buffer.from("Ikuyo")],
	bidProgramId
);

// Associated Token Accounts
export const signerOneATA = getAssociatedTokenAddressSync(
	nftMint,
	signerOneKp.publicKey
);

export const signerTwoATA = getAssociatedTokenAddressSync(
	nftMint,
	signerTwoKp.publicKey
);

export const vaultTokenAccount = getAssociatedTokenAddressSync(
	nftMint,
	assetManager,
	true
);
