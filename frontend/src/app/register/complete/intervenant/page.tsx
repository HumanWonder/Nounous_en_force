"use client";
import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "../../../hooks/useAuth";

import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"

import type { IntervenantFormData } from "../../../types/user";

export default function IntervenantRegister() {
    const router = useRouter();
    const { token, isAuthenticated } = useAuth();
    const [documents, setDocuments] = useState<FileList | null>(null)

    // const [isDropdownOpen, setIsDropdownOpen] = useState(false);

    const [formData, setFormData] = useState<IntervenantFormData>({
        temp_info: {
            last_name: "",
            first_name: "",
            address: "",
            phone: "",
            birth_date: "",
            driver_license: false,
            transport_modes: "",
        },
        availabilities: [{ available_periods: "", work_hours: "", preferred_locations: "", max_travel_time: "" }],
        conditions: [{ hourly_rate: "", contract_types: "", self_employment: "" }],
        diplomas: [{ diploma_name: "", other_certifications: "", year_obtained: "", institution: "" }],
        experiences: [{ total_experience: "", previous_jobs: "", structure_types: "", tasks: "" }],
        skills: [{ languages: "", pedagogies: "", special_skills: "", special_needs_handling: "" }],
        documents: [{ motivation_letter: "", professional_refs: "", required_docs: "", criminal_record: "", diplomas: "" }],
    },
    );

    // const toggleDropdown = () => {
    //     setIsDropdownOpen(!isDropdownOpen);
    // };


    // Charge les données du formulaire si existantes dans localStorage
    useEffect(() => {
        if (typeof window !== "undefined") {
            const storedFormData = localStorage.getItem("formData");
            if (storedFormData) {
                setFormData(JSON.parse(storedFormData));
            }
        }
    }, []);

    //Sauvegarde des données à chaque changement
    useEffect(() => {
        if (typeof window !== "undefined") {
            localStorage.setItem("formData", JSON.stringify(formData));
        }
    }, [formData]);

    const handleChange = (e) => {
        const { name, value, type, checked } = e.target;
        setFormData((prev) => ({
            ...prev,
            temp_info: {
                ...prev.temp_info,
                [name]: type === "checkbox" ? checked : value,
            }
        }));
    };

    const handleArrayChange = (index, field, value, arrayName) => {
        setFormData((prev) => {
            console.log("prev[arrayName] :", prev[arrayName]);
            const updatedArray = [...prev.temp_info[arrayName]];
            updatedArray[index][field] = value;
            return {
                ...prev,
                temp_info: {
                    ...prev.temp_info,
                    [arrayName]: updatedArray
                }
            };
        });
    };

    const addArrayField = (arrayName, emptyObject) => {
        setFormData((prev) => ({
            ...prev,
            temp_info: {
                ...prev.temp_info,
                [arrayName]: [...prev.temp_info[arrayName], emptyObject]
            }
        }));
    };

    const handleSubmit = async (e) => {
        e.preventDefault();
        if (!token || !isAuthenticated) {
            alert("Token inexistant, connexion non authentifiée, redirection vers login");
            return;
        }
        //Data formattée pour correspondre exactement à la struct attendue dans le back-end
        const preparedData = {
            temp_info: {
                last_name: formData.temp_info.last_name,
                first_name: formData.temp_info.first_name,
                address: formData.temp_info.address,
                phone: formData.temp_info.phone,
                birth_date: formData.temp_info.birth_date || null,
                driver_license: formData.temp_info.driver_license,
                transport: formData.temp_info.transport_modes,
            },
            availabilities: formData.availabilities,
            conditions: formData.conditions,
            diplomas: formData.diplomas.map((doc) => ({
                ...doc,
                year_obtained: parseInt(doc.year_obtained, 10)
            })),
            experiences: formData.experiences,
            skills: formData.skills,
            documents: formData.documents,
        };


        console.log("Prepared Data ready to be sent to server : \n", preparedData);
        try {

            const response = await fetch("http://127.0.0.1:8080/register/temp", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    "Authorization": `Bearer ${token}`,
                },
                body: JSON.stringify(preparedData),
            });
            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(errorText);
            }
            router.push("/profile");
        } catch (error) {
            console.error("Erreur lors de l'envoi du formulaire :", error);
        }
    };

    return (
        <form
            onSubmit={handleSubmit}
            className="max-w-4xl mx-auto px-4 py-8 bg-white rounded-2xl shadow-md space-y-10"
        >
            <h1 className="text-3xl font-bold text-center text-primary">Formulaire Intervenant·e / Remplaçant·e</h1>

            {/* A. Informations personnelles */}
            <section className="space-y-4">
                <h2 className="text-xl font-semibold text-gray-700">A. Informations personnelles</h2>
                <div className="grid md:grid-cols-2 gap-4">
                    <Input name="name" required placeholder="Nom et prénom" className="w-full" />
                    <Input name="birthdate" type="date" placeholder="Date de naissance" className="w-full" />
                </div>
                <Input name="address" required placeholder="Adresse complète" className="w-full" />
                <div className="grid md:grid-cols-2 gap-4">
                    <Input name="phone" type="tel" required placeholder="Téléphone" className="w-full" />
                    <Input name="email" type="email" required placeholder="E-mail" className="w-full" />
                </div>
                <div className="flex flex-col gap-2">
                    <Label htmlFor="has_license">Permis de conduire :</Label>
                    <select name="has_license" className="w-full border rounded p-2">
                        <option value="yes">Oui</option>
                        <option value="no">Non</option>
                    </select>
                </div>
                <Input name="transport" placeholder="Moyen de transport" className="w-full" />
            </section>

            {/* B. Formation et diplômes */}
            <section className="space-y-4">
                <h2 className="text-xl font-semibold text-gray-700">B. Formation et diplômes</h2>
                <div className="grid md:grid-cols-2 gap-4">
                    <Input name="main_diploma" required placeholder="Diplôme principal" className="w-full" />
                    <Input name="other_diplomas" placeholder="Autres diplômes / certifications" className="w-full" />
                </div>
                <div className="grid md:grid-cols-2 gap-4">
                    <Input name="graduation_year" type="number" placeholder="Année d'obtention" className="w-full" />
                    <Input name="school" placeholder="Établissement de formation" className="w-full" />
                </div>
            </section>

            {/* C. Expérience pro */}
            <section className="space-y-4">
                <h2 className="text-xl font-semibold text-gray-700">C. Expérience professionnelle</h2>
                <Input name="total_experience" placeholder="Expérience totale en petite enfance" className="w-full" />
                <Textarea name="previous_positions" placeholder="Postes précédents" className="w-full" />
                <Input name="structures" placeholder="Types de structures fréquentées" className="w-full" />
                <Textarea name="tasks" placeholder="Tâches réalisées" className="w-full" />
            </section>

            {/* D. Compétences spécifiques */}
            <section className="space-y-4">
                <h2 className="text-xl font-semibold text-gray-700">D. Compétences spécifiques</h2>
                <Input name="languages" placeholder="Langues parlées" className="w-full" />
                <Input name="pedagogies" placeholder="Pédagogies ou méthodes connues" className="w-full" />
                <Input name="skills" placeholder="Savoir-faire particuliers" className="w-full" />
                <Input name="situations" placeholder="Gestion de situations particulières" className="w-full" />
            </section>

            {/* E. Disponibilités */}
            <section className="space-y-4">
                <h2 className="text-xl font-semibold text-gray-700">E. Disponibilités</h2>
                <div className="grid md:grid-cols-2 gap-4">
                    <Input name="availability_periods" placeholder="Périodes disponibles" className="w-full" />
                    <Input name="time_slots" placeholder="Plages horaires" className="w-full" />
                </div>
                <Input name="zones" placeholder="Zones géographiques acceptées" className="w-full" />
                <Input name="travel_time" placeholder="Temps de trajet maximum" className="w-full" />
            </section>

            {/* F. Rémunération */}
            <section className="space-y-4">
                <h2 className="text-xl font-semibold text-gray-700">F. Rémunération et conditions</h2>
                <Input name="rate" placeholder="Taux horaire / prétentions salariales" className="w-full" />
                <Input name="contract_types" placeholder="Types de contrats acceptés" className="w-full" />
                <Input name="auto_entrepreneur" placeholder="Possibilité de facturation en auto-entreprise" className="w-full" />
            </section>

            {/* G. Autres informations */}
            <section className="space-y-4">
                <h2 className="text-xl font-semibold text-gray-700">G. Autres informations</h2>
                <Textarea name="motivation" placeholder="Mini-lettre de motivation" className="w-full" />
                <Textarea name="references" placeholder="Références professionnelles" className="w-full" />
                <div className="space-y-2">
                    <Label>Documents à fournir :</Label>
                    <Input
                        type="file"
                        name="documents"
                        multiple
                        accept=".pdf,.jpg,.jpeg,.png"
                        onChange={(e) => setDocuments(e.target.files)}
                        className="w-full"
                    />
                    <p className="text-sm text-gray-500">Joindre casier judiciaire, diplômes, attestations...</p>
                </div>
            </section>

            <Button type="submit" className="w-full mt-6">
                Envoyer le formulaire
            </Button>
        </form>

    );
}
