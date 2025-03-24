/*Form pour les intervenant.e.s */
"use client";
import { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function IntervenantRegister() {

    const router = useRouter();

    const [formData, setFormData] = useState({
        full_name: '',
        address: '',
        phone: '',
        email: '',
        birth_date: '',
        driver_license: false,
        transport: '',
        motivation: '',
        judicial_record: ''
    });

    const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
        const { name, value } = e.target;
        setFormData({ ...formData, [name]: value });
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        try {
            const response = await fetch("/register/temp", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(formData),
            });

            const data = await response.json();
            console.log(data);
            alert("Inscription enregistrée, en attente de validation par un administrateur.");
            // Rediriger l'utilisateur vers la page d'accueil
            router.push("/");
        } catch (error) {
            console.error("Erreur lors de l'envoi du formulaire :", error);
        }
    };
// A debug : formulaire type et réponse du serveur

    return (
        <form onSubmit={handleSubmit}>
            <input
                type="text"
                name="full_name"
                value={formData.full_name}
                onChange={handleChange}
                placeholder="Nom complet"
            />
            <input
                type="text"
                name="address"
                value={formData.address}
                onChange={handleChange}
                placeholder="Adresse"
            />
            <input
                type="text"
                name="phone"
                value={formData.phone}
                onChange={handleChange}
                placeholder="Téléphone"
            />
            <input
                type="email"
                name="email"
                value={formData.email}
                onChange={handleChange}
                placeholder="Email"
            />
            <input
                type="date"
                name="birth_date"
                value={formData.birth_date}
                onChange={handleChange}
            />
            <input
                type="checkbox"
                name="driver_license"
                checked={formData.driver_license}
                onChange={() => setFormData({ ...formData, driver_license: !formData.driver_license })}
            />
            <input
                type="text"
                name="transport"
                value={formData.transport}
                onChange={handleChange}
                placeholder="Mode de transport"
            />
            <textarea
                name="motivation"
                value={formData.motivation}
                onChange={handleChange}
                placeholder="Motivation"
            />
            <input
                type="text"
                name="judicial_record"
                value={formData.judicial_record}
                onChange={handleChange}
                placeholder="Casier judiciaire"
            />
            <button type="submit">Envoyer</button>
        </form>
    );
}