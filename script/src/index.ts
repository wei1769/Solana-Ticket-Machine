import { publicKey, struct, u32, u64, u8 } from "@project-serum/borsh";
import {
  Account,
  AccountMeta,
  Connection,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { TokenAmount } from "./math";

// TODO
const manager_private_key = [];
const managerAccount = new Account(manager_private_key);

const POOL_ID = "D9ioyVKEQkjbEpQFcQPDHQkTCfuKJU8QLzN6xcbr7LAe";
const TICKET_PUBLIC_KEY = "AUaGuQhpjttMdBmejoboMoUMrpcxNHZsT44C6jupLYNP";
const FEE_RECEIVER_PUBLIC_KEY = "2wnEcArzCpX1QRdtpHRXxZ7k9b1UeK16mPt26LPWFZ6V";

// devnet connection
const connection = new Connection(
  "https://api.devnet.solana.com",
  "singleGossip"
);

// async function getLamports() {
//   const accountInfo = await connection.getAccountInfo(
//     new PublicKey(POOL_MANAGER_PUBLIC_KEY)
//   );
//   const { lamports } = accountInfo;
//   console.log("lamports", lamports);
// }

// referenced from program/src/state.rs Pool struct
const POOL_LAYOUT = struct([
  u8("account_type"),
  u32("manager"),
  publicKey("fee_reciever"),
  u64("total_amount"),
  u64("price"),
  u8("fee"),
  u64("current_number"),
]);

// referenced from program/src/state.rs Ticket struct
const TICKET_LAYOUT = struct([
  u8("account_type"),
  publicKey("pool_id"),
  u64("ticketnumber"),
  publicKey("ticketbuyer"),
]);

// let { transaction, poolKeyPair } = initPool();
export async function initPool(
  managerAccount: Account,
  price: number,
  fee: number,
  total_amount: number
) {
  // ticket program Id is hard-coded
  const ticketProgramId = new PublicKey(TICKET_PUBLIC_KEY);
  // console.log("ticketProgramId", ticketProgramId);

  // create new public key for pool
  const newPoolAccount = new Account();
  let poolPublicKey = newPoolAccount.publicKey;
  // console.log("poolPublicKey", poolPublicKey);

  // add create account instruction to transaction
  const transaction = new Transaction();
  transaction.add(
    SystemProgram.createAccount({
      fromPubkey: managerAccount.publicKey,
      newAccountPubkey: poolPublicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(
        POOL_LAYOUT.span
      ),
      space: POOL_LAYOUT.span,
      programId: ticketProgramId,
    })
  );

  // prepare keys
  const keys: AccountMeta[] = [
    { pubkey: poolPublicKey, isSigner: true, isWritable: true },
    { pubkey: managerAccount.publicKey, isSigner: true, isWritable: true },
    {
      pubkey: new PublicKey(FEE_RECEIVER_PUBLIC_KEY),
      isSigner: false,
      isWritable: true,
    },
  ];

  // prepare data
  // init pool layout, referenced from program/src/instruction.rs
  const dataLayout = struct([
    u8("instruction"),
    u64("price"),
    u8("fee"),
    u64("total_amount"),
  ]);
  const data = Buffer.alloc(dataLayout.span);
  dataLayout.encode(
    {
      instruction: 0,
      price: new TokenAmount(price),
      fee: new TokenAmount(fee),
      total_amount: new TokenAmount(total_amount),
    },
    data
  );

  // add init pool instruction to transaction
  transaction.add(
    new TransactionInstruction({
      keys,
      programId: ticketProgramId,
      data,
    })
  );

  // return { transaction, newPoolAccount };

  await sendAndConfirmTransaction(
    connection,
    transaction,
    [newPoolAccount, managerAccount],
    {
      skipPreflight: false,
      commitment: "recent",
      preflightCommitment: "recent",
    }
  );
}

// let { transaction, ticketKeyPair } = buy();
export async function buy(pool_id: string, buyer: Account) {
  const ticketProgramId = new PublicKey(TICKET_PUBLIC_KEY);

  // create new public key for ticket
  const newTicketAccount = new Account();
  let ticketPublicKey = newTicketAccount.publicKey;

  // add create account instruction to transaction
  const transaction = new Transaction();
  transaction.add(
    SystemProgram.createAccount({
      fromPubkey: buyer.publicKey,
      newAccountPubkey: ticketPublicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(
        TICKET_LAYOUT.span
      ),
      space: TICKET_LAYOUT.span,
      programId: ticketProgramId,
    })
  );

  // get pool info
  let poolInfo = await connection.getAccountInfo(new PublicKey(pool_id));
  console.log("poolInfo", poolInfo);

  // add buy instruction to transaction

  // return { transaction, newTicketAccount };
  await sendAndConfirmTransaction(connection, transaction, [newTicketAccount], {
    skipPreflight: false,
    commitment: "recent",
    preflightCommitment: "recent",
  });
}

// I'm the pool manager, so fill in my own public key
let manager = managerAccount;
let price = 696969;
let fee = 23;
let amount = 10;
initPool(manager, price, fee, amount);

let pool_id = POOL_ID;
let buyer = managerAccount;
buy(pool_id, buyer);
