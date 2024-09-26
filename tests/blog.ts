import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Blog } from "../target/types/blog";
// import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

describe("blog", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();

  const program = anchor.workspace.Blog as Program<Blog>;

  it("Is initialized!", async () => {
    // Add your test here.
    const name = "test";
    const avatar = "ajdljaldjaldjal aldjald";

    await program.methods.initUser(name, avatar).rpc();
    const data = await program.account.userAccount.all();
    console.log("Your transaction signature", data);

    await program.methods.createPost("name", "avatar").rpc();
    await program.methods.createPost("name", "avatar").rpc();

    const post = await program.account.postAccount.all();
    console.log("Your transaction signature", post);


    // const seeds = [
    //   Buffer.from("post"), // Replace with the actual seed string
    //   provider.publicKey.toBuffer(), // Owner's public key
    //   Buffer.from(new Uint8Array([0]).buffer), // Convert last_post_id to bytes
    // ];
    // // const seeds = [
    // //   utf8.encode("user"), // Replace with the actual seed string
    // //   provider.publicKey.toBuffer(), // Owner's public key
    // //   // Buffer.from(new Uint32Array([1]).buffer), // Convert last_post_id to bytes
    // // ];
    // const add = anchor.web3.PublicKey.findProgramAddressSync(
    //   seeds,
    //   program.programId
    // );
    // console.log(add);
  });
});



// A5b6i6qrxc5JAkLjnmrbrp17GwUkQSFXf3NKcY87AsKv