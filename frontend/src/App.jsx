import { useState, useEffect } from 'react';
import axios from 'axios';
import './App.css';

function App() {
  const [products, setProducts] = useState([]);
  const [productForm, setProductForm] = useState({ sku: '', name: '', price: 0 });
  const [stockForm, setStockForm] = useState({ sku: '', quantity: 0 });
  const [orderForm, setOrderForm] = useState({ items: [{ sku: '', quantity: 0 }], total: 0 });

  useEffect(() => {
    fetchProducts();
  }, []);

  const fetchProducts = async () => {
    try {
      const res = await axios.get('http://localhost:8081/products');
      setProducts(res.data);
    } catch (e) {
      console.error('Error fetching products:', e);
    }
  };

  const handleProductSubmit = async (e) => {
    e.preventDefault();
    try {
      await axios.post('http://localhost:8081/products', {
        sku: productForm.sku,
        name: productForm.name,
        price: parseFloat(productForm.price),
      });
      fetchProducts();
      setProductForm({ sku: '', name: '', price: 0 });
    } catch (e) {
      console.error('Error creating product:', e);
    }
  };

  const handleStockSubmit = async (e) => {
    e.preventDefault();
    try {
      await axios.patch(`http://localhost:8082/inventory/${stockForm.sku}`, {
        quantity: parseInt(stockForm.quantity),
      });
      alert('Stock updated');
      setStockForm({ sku: '', quantity: 0 });
    } catch (e) {
      console.error('Error updating stock:', e);
    }
  };

  const handleOrderSubmit = async (e) => {
    e.preventDefault();
    try {
      await axios.post('http://localhost:8083/orders', {
        items: orderForm.items,
        total: parseFloat(orderForm.total),
      });
      alert('Order placed');
      setOrderForm({ items: [{ sku: '', quantity: 0 }], total: 0 });
    } catch (e) {
      console.error('Error placing order:', e);
    }
  };

  return (
    <div>
      <h1>E-commerce Platform Test</h1>

      <h2>Create Product</h2>
      <form onSubmit={handleProductSubmit}>
        <input
          type="text"
          placeholder="SKU"
          value={productForm.sku}
          onChange={(e) => setProductForm({ ...productForm, sku: e.target.value })}
        />
        <input
          type="text"
          placeholder="Name"
          value={productForm.name}
          onChange={(e) => setProductForm({ ...productForm, name: e.target.value })}
        />
        <input
          type="number"
          placeholder="Price"
          value={productForm.price}
          onChange={(e) => setProductForm({ ...productForm, price: e.target.value })}
        />
        <button type="submit">Add Product</button>
      </form>

      <h2>Update Stock</h2>
      <form onSubmit={handleStockSubmit}>
        <input
          type="text"
          placeholder="SKU"
          value={stockForm.sku}
          onChange={(e) => setStockForm({ ...stockForm, sku: e.target.value })}
        />
        <input
          type="number"
          placeholder="Quantity"
          value={stockForm.quantity}
          onChange={(e) => setStockForm({ ...stockForm, quantity: e.target.value })}
        />
        <button type="submit">Update Stock</button>
      </form>

      <h2>Place Order</h2>
      <form onSubmit={handleOrderSubmit}>
        <input
          type="text"
          placeholder="Item SKU"
          value={orderForm.items[0].sku}
          onChange={(e) => {
            const items = [...orderForm.items];
            items[0].sku = e.target.value;
            setOrderForm({ ...orderForm, items });
          }}
        />
        <input
          type="number"
          placeholder="Quantity"
          value={orderForm.items[0].quantity}
          onChange={(e) => {
            const items = [...orderForm.items];
            items[0].quantity = parseInt(e.target.value);
            setOrderForm({ ...orderForm, items });
          }}
        />
        <input
          type="number"
          placeholder="Total"
          value={orderForm.total}
          onChange={(e) => setOrderForm({ ...orderForm, total: e.target.value })}
        />
        <button type="submit">Place Order</button>
      </form>

      <h2>Products</h2>
      <ul>
        {products.map((p) => (
          <li key={p.product_id}>
            {p.name} (SKU: {p.sku}, Price: ${p.price})
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;