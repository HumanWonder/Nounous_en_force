"use client";
import { useState } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "../../../hooks/useAuth";

export default function IntervenantRegister() {
    const router = useRouter();
    const { token, isAuthenticated } = useAuth();
    // const [isDropdownOpen, setIsDropdownOpen] = useState(false);

    const [formData, setFormData] = useState({
        temp_info: {
            full_name: "",
            address: "",
            phone: "",
            birth_date: "",
            driver_license: false,
            transport: "",
            motivation: "",
            judicial_record: "",
            availabilities: [{ available_periods: "", work_hours: "", preferred_locations: "", max_travel_time: "" }],
            conditions: [{ hourly_rate: "", contract_types: "", self_employment: null }],
            diplomas: [{ diploma_name: "", other_certifications: "", year_obtained: "", institution: "" }],
            experiences: [{ total_experience: "", previous_jobs: "", structure_types: "", tasks: "" }]
        }
    });

    // const toggleDropdown = () => {
    //     setIsDropdownOpen(!isDropdownOpen);
    // };

    const handleChange = (e) => {
        const { name, value, type, checked } = e.target;
        setFormData((prev) => ({
            ...prev,
            temp_info: {
                ...prev.temp_info,
                [name]: type === "checkbox" ? checked : value,
            },
        }));
    };

    const handleArrayChange = (index, field, value, arrayName) => {
        setFormData((prev) => {
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
        try {
            const formDataToSend = JSON.stringify(formData);
            console.log(formDataToSend);
            const response = await fetch("http://127.0.0.1:8080/register/temp", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    "Authorization": `Bearer ${token}`,
                },
                body: formDataToSend,
            });
            const data = await response.json();
            alert("Inscription enregistrée, en attente de validation par un administrateur.");
            router.push("/");
        } catch (error) {
            console.error("Erreur lors de l'envoi du formulaire :", error);
        }
    };

    return (
        <div className="max-w-none mx-auto p-6 bg-white shadow-md rounded-lg">
            <h2 className="text-xl font-semibold mb-4">Inscription Intervenant.e</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                {/* Première colonne */}
                <div className="flex flex-col gap-4">
                    <input className="input-field border p-2" type="text" name="full_name" value={formData.temp_info.full_name} onChange={handleChange} placeholder="Nom complet" />
                    <input className="input-field border p-2" type="text" name="address" value={formData.temp_info.address} onChange={handleChange} placeholder="Adresse" />
                    <input className="input-field border p-2" type="text" name="phone" value={formData.temp_info.phone} onChange={handleChange} placeholder="Téléphone" />
                    <input className="input-field border p-2" type="date" name="birth_date" value={formData.temp_info.birth_date} onChange={handleChange} />
                    <div className="flex items-center gap-2">
                        <input type="checkbox" name="driver_license" checked={formData.temp_info.driver_license} onChange={handleChange} />
                        <label>Possède un permis de conduire</label>
                    </div>
                    <input className="input-field border p-2" type="text" name="transport" value={formData.temp_info.transport} onChange={handleChange} placeholder="Mode de transport" />
                    <textarea className="input-field border p-2" name="motivation" value={formData.temp_info.motivation} onChange={handleChange} placeholder="Motivation"></textarea>
                    <input className="input-field border p-2" type="text" name="judicial_record" value={formData.temp_info.judicial_record} onChange={handleChange} placeholder="Casier judiciaire" />
                </div>

                {/* Deuxième colonne */}
                <div className="flex flex-col gap-4">
                    {/* Disponibilités dynamiques */}
                    <h3 className="text-lg font-semibold">Disponibilités</h3>
                    {formData.temp_info.availabilities.map((availability, index) => (
                        <div key={index} className="border p-2 rounded">
                            <input className="input-field border p-2" type="text" value={availability.available_periods} onChange={(e) => handleArrayChange(index, "available_periods", e.target.value, "availabilities")} placeholder="Périodes disponibles (ex: Lundi-Vendredi)" />
                            <input className="input-field border p-2" type="text" value={availability.work_hours} onChange={(e) => handleArrayChange(index, "work_hours", e.target.value, "availabilities")} placeholder="Horaires disponibles (ex: 9h-18h)" />
                            <input className="input-field border p-2" type="text" value={availability.preferred_locations} onChange={(e) => handleArrayChange(index, "preferred_locations", e.target.value, "availabilities")} placeholder="Lieux souhaités" />
                            <input className="input-field border p-2" type="text" value={availability.max_travel_time} onChange={(e) => handleArrayChange(index, "max_travel_time", e.target.value, "availabilities")} placeholder="Temps de déplacement max (ex: 30min)" />
                        </div>
                    ))}
                    <button type="button" className="btn-secondary" onClick={() => addArrayField("availabilities", { available_periods: "", work_hours: "", preferred_locations: "", max_travel_time: "" })}>
                        Ajouter une disponibilité
                    </button>

                    {/* Conditions de travail souhaitées */}
                    <h3 className="text-lg font-semibold">Conditions de travail</h3>
                    {formData.temp_info.conditions.map((conditions, index) => (
                        <div key={index} className="border p-2 rounded">
                            <input className="input-field border p-2" type="text" value={conditions.contract_types} onChange={(e) => handleArrayChange(index, "contract_types", e.target.value, "conditions")} placeholder="Type de contrat souhaité" />
                            <input className="input-field border p-2" type="text" value={conditions.hourly_rate} onChange={(e) => handleArrayChange(index, "hourly_rate", e.target.value, "conditions")} placeholder="Horaires de travail souhaités (ex: 9h-18h)" />
                            <select className="input-field border p-2" name="self-employment" value={conditions.self_employment === null ? "" : conditions.self_employment ? "true" : "false"} onChange={(e) => handleArrayChange(index, "self_employment", e.target.value === "true", "conditions")}>
                                <option value="">{/* Valeur par défaut*/}Souhaitez-vous faire de l'auto-entreprenariat ?</option>
                                <option value="true">Oui</option>
                                <option value="false">Non</option>
                            </select>
                        </div>
                    ))}

                    {/* Expériences dynamiques */}
                    <h3 className="text-lg font-semibold">Expériences</h3>
                    {formData.temp_info.experiences.map((experience, index) => (
                        <div key={index} className="border p-2 rounded">
                            <input className="input-field border p-2" type="text" value={experience.total_experience} onChange={(e) => handleArrayChange(index, "total_experience", e.target.value, "experiences")} placeholder="Expérience totale" />
                            <input className="input-field border p-2" type="text" value={experience.previous_jobs} onChange={(e) => handleArrayChange(index, "previous_jobs", e.target.value, "experiences")} placeholder="Précédents emplois" />
                            <input className="input-field border p-2" type="text" value={experience.structure_types} onChange={(e) => handleArrayChange(index, "structure_types", e.target.value, "experiences")} placeholder="Types de structures" />
                            <input className="input-field border p-2" type="text" value={experience.tasks} onChange={(e) => handleArrayChange(index, "tasks", e.target.value, "experiences")} placeholder="Tâches réalisées" />
                        </div>
                    ))}
                    <button type="button" className="btn-secondary" onClick={() => addArrayField("experiences", { total_experience: "", previous_jobs: "", structure_types: "", tasks: "" })}>
                        Ajouter une expérience
                    </button>

                    {/* Diplômes dynamiques */}
                    <h3 className="text-lg font-semibold">Diplômes</h3>
                    {formData.temp_info.diplomas.map((diploma, index) => (
                        <div key={index} className="border p-2 rounded">
                            <input className="input-field border p-2" type="text" value={diploma.diploma_name} onChange={(e) => handleArrayChange(index, "diploma_name", e.target.value, "diplomas")} placeholder="Nom du diplôme" />
                            <input className="input-field border p-2" type="text" value={diploma.other_certifications} onChange={(e) => handleArrayChange(index, "other_certifications", e.target.value, "diplomas")} placeholder="Autres certifications" />
                            <input className="input-field border p-2" type="number" value={diploma.year_obtained} onChange={(e) => handleArrayChange(index, "year_obtained", e.target.value, "diplomas")} placeholder="Année d'obtention" />
                            <input className="input-field border p-2" type="text" value={diploma.institution} onChange={(e) => handleArrayChange(index, "institution", e.target.value, "diplomas")} placeholder="Institution" />
                        </div>
                    ))}
                    <button type="button" className="btn-secondary" onClick={() => addArrayField("diplomas", { diploma_name: "", other_certifications: "", year_obtained: "", institution: "" })}>
                        Ajouter un diplôme
                    </button>
                </div>
            </div>
            {/* Bouton de soumission centré sous les colonnes */}
            <div className="flex justify-center mt-6 border">
                <button type="submit" onClick={handleSubmit} className="btn-primary">Envoyer</button>
            </div>
        </div>
    );
}
