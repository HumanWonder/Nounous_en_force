"use client";

import { useRouter } from "next/navigation";
import Button from "./components/button";

export default function Home() {
    const router = useRouter();

    return (
        <div className="">
            <h1>Bienvenue</h1>
            <br/>
            <div>
                <p>Denique Antiochensis ordinis vertices sub uno elogio iussit occidi ideo efferatus, quod ei celebrari vilitatem intempestivam urgenti, cum inpenderet inopia, gravius rationabili responderunt; et perissent ad unum ni comes orientis tunc Honoratus fixa constantia restitisset.

Vbi curarum abiectis ponderibus aliis tamquam nodum et codicem difficillimum Caesarem convellere nisu valido cogitabat, eique deliberanti cum proximis clandestinis conloquiis et nocturnis qua vi, quibusve commentis id fieret, antequam effundendis rebus pertinacius incumberet confidentia, acciri mollioribus scriptis per simulationem tractatus publici nimis urgentis eundem placuerat Gallum, ut auxilio destitutus sine ullo interiret obstaculo.

Cum haec taliaque sollicitas eius aures everberarent expositas semper eius modi rumoribus et patentes, varia animo tum miscente consilia, tandem id ut optimum factu elegit: et Vrsicinum primum ad se venire summo cum honore mandavit ea specie ut pro rerum tunc urgentium captu disponeretur concordi consilio, quibus virium incrementis Parthicarum gentium a arma minantium impetus frangerentur.</p>
            </div>

            {/* <Button onClick={() => router.push("/register/owner")}>
                Inscription Responsable de créche
            </Button>
            <Button onClick={() => router.push("/register/temp")}>
                Inscription Intervenant.e
            </Button> */}
            <br/>
            <Button onClick={() => router.push("/login")}>Connectez-vous!</Button>
        </div>
    );
}