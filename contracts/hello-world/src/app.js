// Configura la librería de Soroban
import { Client, Keypair, TransactionBuilder } from 'soroban-sdk';

// Tu dirección pública (ID) y clave secreta (private key)
const publicKey = "GAYB4T4CJR6FLY3KZZTISRQGDSYYZ4IZWCKZQLXNXQFTBKRYVJN5UZRV";
const secretKey = "SDR6MLTJ5QICKNNIDSC4Y4XJXEVCNOFRF5C73Q6B465Y6ZI4IN3QPISU";

// Crear un cliente de Soroban para testnet
const client = new Client("https://soroban-testnet.stellar.org");

// Crear el keypair de la cuenta a partir de la clave secreta
const keypair = Keypair.fromSecret(secretKey);

// Configuración de tu contrato desplegado
const contractId = "CDDM6J2DPYD3QNVAYZEUYTOQG3L6UA4QTEMUFZCECLUAQTWPUVARLHNW";

// Función para invocar el contrato y agregar un producto
async function addProduct(name, quantity, price) {
  try {
    const tx = new TransactionBuilder()
      .addOperation({
        type: "invoke_contract",
        contractId,
        method: "add_product",
        args: [name, quantity, price],
        source: publicKey,
      })
      .build();
    const signedTx = await keypair.sign(tx);
    const response = await client.submitTransaction(signedTx);
    console.log("Product added:", response);
  } catch (error) {
    console.error("Error adding product:", error);
  }
}

// Función para obtener un producto
async function getProduct(name) {
  try {
    const response = await client.invokeContract(contractId, "get_product", [name]);
    console.log("Product info:", response);
  } catch (error) {
    console.error("Error getting product:", error);
  }
}

// Función para actualizar un producto
async function updateProduct(name, newQuantity, newPrice) {
  try {
    const tx = new TransactionBuilder()
      .addOperation({
        type: "invoke_contract",
        contractId,
        method: "update_product",
        args: [name, newQuantity, newPrice],
        source: publicKey,
      })
      .build();
    const signedTx = await keypair.sign(tx);
    const response = await client.submitTransaction(signedTx);
    console.log("Product updated:", response);
  } catch (error) {
    console.error("Error updating product:", error);
  }
}

// Función para eliminar un producto
async function deleteProduct(name) {
  try {
    const tx = new TransactionBuilder()
      .addOperation({
        type: "invoke_contract",
        contractId,
        method: "delete_product",
        args: [name],
        source: publicKey,
      })
      .build();
    const signedTx = await keypair.sign(tx);
    const response = await client.submitTransaction(signedTx);
    console.log("Product deleted:", response);
  } catch (error) {
    console.error("Error deleting product:", error);
  }
}

// Función para saludar (ejemplo)
async function hello(to) {
  try {
    const response = await client.invokeContract(contractId, "hello", [to]);
    console.log("Hello message:", response);
  } catch (error) {
    console.error("Error saying hello:", error);
  }
}

// Prueba de funciones
addProduct("Pasta", 10, 200);
getProduct("Pasta");
updateProduct("Pasta", 15, 250);
deleteProduct("Pasta");
hello("Mundo");
