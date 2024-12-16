const StellarSdk = require('stellar-sdk');
const axios = require('axios');

const serverUrl = 'https://horizon-testnet.stellar.org';
const sourceKeys = StellarSdk.Keypair.fromSecret('SAZUTYM3OK7JCFE5EF57RUOHGDCR7TRMF3DGABYBQTGS2UXGKDP2M5XE'); // Clave secreta
const contractId = 'CBEMDMOMGPPQVJ3MGLH5EBMAXWWEFR2WNQ52RMQJYBHWSOB5BDGTXDDM'; // ID del contrato

// Botones de interfaz
document.getElementById('addDishBtn').addEventListener('click', addDish);
document.getElementById('deleteDishBtn').addEventListener('click', deleteDish);
document.getElementById('updateDishBtn').addEventListener('click', updateDish);
document.getElementById('getMenuBtn').addEventListener('click', getMenu);

// Función común para enviar operaciones al contrato
async function sendTransaction(operation, params) {
    try {
        const server = new StellarSdk.Server(serverUrl);
        const account = await server.loadAccount(sourceKeys.publicKey());

        const tx = new StellarSdk.TransactionBuilder(account, {
            fee: StellarSdk.BASE_FEE,
            networkPassphrase: StellarSdk.Networks.TESTNET,
        })
        .addOperation(StellarSdk.Operation.invokeContractFunction({
            contract: contractId,
            function: operation,
            args: params
        }))
        .setTimeout(100)
        .build();

        tx.sign(sourceKeys);
        const response = await server.submitTransaction(tx);
        console.log(`Operación ${operation} ejecutada:`, response);
    } catch (error) {
        console.error(`Error en ${operation}:`, error);
    }
}

// Agregar un platillo
async function addDish() {
    const name = document.getElementById('dishName').value;
    const stock = parseInt(document.getElementById('dishStock').value);
    const price = parseInt(document.getElementById('dishPrice').value);

    if (!name || isNaN(stock) || isNaN(price)) {
        console.error('Por favor, ingresa los datos del platillo correctamente.');
        return;
    }
    await sendTransaction('add_dish', [name, stock, price]);
    getMenu();
}

// Eliminar un platillo
async function deleteDish() {
    const name = document.getElementById('dishName').value;

    if (!name) {
        console.error('Por favor, proporciona el nombre del platillo.');
        return;
    }
    await sendTransaction('delete_dish', [name]);
    getMenu();
}

// Actualizar un platillo
async function updateDish() {
    const name = document.getElementById('dishName').value;
    const stock = parseInt(document.getElementById('dishStock').value);
    const price = parseInt(document.getElementById('dishPrice').value);

    if (!name || isNaN(stock) || isNaN(price)) {
        console.error('Por favor, ingresa los datos correctamente.');
        return;
    }
    await sendTransaction('update_dish', [name, stock, price]);
    getMenu();
}

// Obtener el menú completo
async function getMenu() {
    try {
        console.log('Obteniendo el menú...');
        const response = await axios.get('/get_menu'); // Simulación de una API Flask o servidor
        const dishes = response.data;

        const menuList = document.getElementById('menuList');
        menuList.innerHTML = '';
        dishes.forEach(dish => {
            const item = document.createElement('li');
            item.textContent = `Platillo: ${dish[0]}, Stock: ${dish[1]}, Precio: ${dish[2]} MXN`;
            menuList.appendChild(item);
        });
    } catch (error) {
        console.error('Error al obtener el menú:', error.message);
    }
}
