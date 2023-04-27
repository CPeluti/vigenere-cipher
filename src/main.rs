use std::{fs, vec, slice};
use std::collections::HashMap;
use regex::Regex;

fn cipher(text: &str, key: &str) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut ciphred_text = String::from("");
    for (i,letter) in text.chars().enumerate() {
        // println!("{}", letter as u32-97);
        let letter_value = letter as u32-97;
        let key_letter = key.as_bytes()[i%key.len()] as u32-97;
        let new_index = (letter_value+key_letter)%(alphabet.len() as u32);
        let ciphred_letter = alphabet.chars().nth(new_index.try_into().unwrap()).unwrap();
        ciphred_text.push(ciphred_letter);
    }
    return ciphred_text;
}

fn decipher(text: &str, key: &str) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut deciphred_text = String::from("");
    let mut counter_non_chars = 0;
    for (i, letter) in text.chars().enumerate() {
        if ![' ', '\n', ';', '\'', '—','-',',','.'].contains(&letter){
            let index = i-counter_non_chars;
            let let_value = letter as i32-97;
            let key_let = key.as_bytes()[index%key.len()] as i32-97;
            let new_index = (let_value-key_let + (alphabet.len()) as i32) % (alphabet.len()) as i32;
            
            let deciphred_letter = alphabet.chars().nth(new_index.try_into().unwrap()).unwrap();
            deciphred_text.push(deciphred_letter);
        } else {
            deciphred_text.push(letter);
            counter_non_chars+=1;
        }
    }
    return deciphred_text;
}
fn ioc(text: &str) -> f64{
    let mut letter_freq_in_text: HashMap<char,i32> = HashMap::new();
    let mut n = 0;
    for letter in text.chars(){
        let l_value = letter as i32-97;
        if l_value>=0 && l_value<=25{
            letter_freq_in_text.entry(letter).and_modify(|value| *value += 1).or_insert(1);
        }
    }
    let mut total = 0;
    for (_, freq) in letter_freq_in_text {
        total += freq*(freq-1);
        n += freq
    }
    return (26.0*n as f64)/((total*(total-1)) as f64);

}

fn calculateLMC(n: i32) -> Vec<i32>{
    let mut lmc: Vec<i32> = vec![];
    for i in 0..n {
        if n % i == 0 {
            lmc.push(i);
        }
    }
    return lmc;
}

fn challenge(letter_freq: &str, text: &str){
    let rgx = Regex::new(r"^[a-z]").unwrap();
    let text_formated = rgx.replace_all(text, "");
    let  mut dict_tri: HashMap<String, (i32, Vec<i32>)> = HashMap::new();
    for i in 0..(text_formated.len()-2) {
        let mut trigrama = String::new();
        trigrama.push(text_formated.chars().nth(i).unwrap());
        trigrama.push(text_formated.chars().nth(i+1).unwrap());
        trigrama.push(text_formated.chars().nth(i+2).unwrap());

        dict_tri.entry(trigrama).and_modify(|(value, vector)| {
            *value += 1;
            vector.push(i as i32);
        }).or_insert((1, vec![]));
    }
    let dict = dict_tri.iter().filter(|&(s, (value, _))| *value > 1);

    // Como fazer assim?
    // let mut dict_freq: (String, i32) = dict.map(|(s, (v, vect))| {
    //     let sub = vect[1] - vect[0];
    //     return (*s, sub);
    // }).collect::<(String,i32)>();'
    
    let mut dict_freq = HashMap::new();
    for (key, (_, pos)) in dict {
        let sub = pos[1] - pos[0];
        let divisores: Vec<i32> = calculateLMC(sub);
        for i in divisores {
            dict_freq.entry(i).and_modify(|value| *value += 1).or_insert(1);
        }
    }
    // dict_freq.iter().max_by(|(a, b)| a.1)
    
}

fn main() {
    let letter_freq_pt_br = "aeosrindmutclpvghqbfzjxkwy";
    let letter_freq_eng = "etaoinshrdlcumwfgypbvkjxqz";
    let desafio = "rvgllakieg tye tirtucatzoe.  whvnvvei i
    winu mpsecf xronieg giid abfuk thv mfuty; wyenvvvr ik ij a drmg,
    drzzqly eomemsei in dy jouc; wyenvvvr i wied mpsvlf znmollnkarzlp
    palszng seworv cfffzn narvhfusvs, rnd srzngznx up khv rerr ff emeiy
    flnvrac i deek; aed ejpvcirlcy wyeeevvr dy hppfs gvt jucy ae upgei
    haed ff mv, tyat zt ieqliies r skroeg dorrl grieczplv tf prvvvnt de
    wrod dvliseiatvlp stvpginx ieto khv stievt, aed detyouicrlcy keotkieg
    geoglv's hrtj ofw--tyen, z atcolnk it yixh tzmv to xek to jer as jofn
    aj i tan.  khzs ij mp susskitltv foi pzstfl rnd sacl.  wzty a
    pyicosfpyicrl wlolrzsh tako tyrfws yidsecf lpoe hzs snoid; i huzetcy
    kakv tf thv syip.  khvre zs eotyieg slrgrijieg ie tyis.  zf khep blt
    keen it, rldosk acl mvn zn tyezr dvgiee, jode tzmv or ftyer, thvrijh
    merp nvarcy khe jade fvecinxs kowrrus tye fcern nity mv.";
    let desafio2 = "tpsja kexis ttgztpb wq ssmil tfdxf vsetw ytafrttw btzf pcbroxdzo zn tqac wix, bwfd s, je ahvup sd pcbqqxff lfzed d avu ytwoxavneh sg p aznst qaghv. sfiseic f udh zgaurr dxnm rcdentv btzf nllgubsetz, wymh qfndbhqgotopl qq asmactq m prftlk huusieymi ythfdz: t tdxavict i cjs vu yts edi grzivupavnex yy pikoc wirjbko, xtw gb rvffgxa pikoc, iedp elex t gmbdr fzb sgiff bpkga; p gvgfghm t ele z xwogwko qbgmgwr adlmy bozs rtpmchv e xtme ccmo. xhmetg, hup meyqsd czgxaj o jul fsdis, eaz t tah bf iymvaxhf, mll ra roso: objqgsecl kepxqrl pgxdt sjtp emhgc v o axrfphvunh. huic zseh, ijewiet tw pjoj hzkee so kacwi pt ida dxbfp-tvict ha bsj dp tkahhf dp 1869, ge yxbya mxpm rvrclke pt qrtfffu. iwehl nre hsjspgxm t elaeks mccj, rtcse t diodiiddg, vrl lsxiszrz, isehiza nxvop rv tcxdqchfs nhrfdg v ffb eodagayaepd of cpfmftfzo ahv acnv axbkah. cezp tquvcj! vpkhmss v qfx rmd vfugx gmghrs yxq mciecthw. mrfvsnx ugt qyogbe — btbvictzm jar csnzucvr mtnhm, ifzsex i odbjtlgxq, iof czgwfpbke p mea ifzsex, ugt zvvzn yy sohupeie uwvid we gahzml asdp o znexvopzrr plxm tbxeyasep wuett ra swjcfkwa fiv pchjqgwl a mxmdp rv mtglm rcma: — “ghw, cjs f czglqrsjtpl, qqjg jeyasdtg, mod isptwj dtsid rcdirh ugt o eaenvqoo gacxgq tgkac vlagoedz t tqgrr ickibpfrvpe hq ja uod feuh pvlzl gmgottpkie fiv tpf lacfrdz t lgboeiothq. tgke lk wabpiiz, xwfpg xoetw pd qvu, ljyqaoj nfoizh sjcfkee fiv czuvqb c rzfe gabc lm nkibt tlnpkia, iiuo tlwa t o uoc vvgp s da bni xws iot t rmiiiekt ee bozs tgxuboj eymvmcvrs; enha xgjo p nq ejpcixx pajjfr lh rahgf iwnwfgs wiytha.” qcd e qbix pazgz! gea, cof mp tvdtdvnoh hmh jznex ebdzzcpl ugt zye oxmjtw. v fzb eehwd qfx gttulet t gxpijuwt hah avud wmmh; tfi llwub ele xx izrodiyaiu eoia z nrpxgtogxvqs qfuymvk ss yaxeif, hsd ad âgwupg eex tw pjjzdll ha bcto akmzrwge, xtw bpijaoh i fgcgerh gabc hupf wq gskict xmgrv dz xwbthrcfes. fpfue p tfagfvctws. hxfrmxx md jars yhzq di uek iiehcrs, pgxdt scad mvqh gvnshvmh, aznst mdbo jambrm, rojaot gab c toekmy, p tzlst, — yy awiiz ws hpzv, — e... exrtpa ganbizrwr! dljyu p dfunh pttg uicxm cjsd ect e ftftetke etbyoct. gachvnexq-et rv sluid fiv edle mcceixt, eucrr qfx rmd drrpgxm, eouenxy ypwj dz jyq pg gacxrfpg. v vpkhmss, gaoxgqj arid. gea swxo bni et qrrabwet, bro obka fiv sp wiumojsp ksxpf gewh gtpc, toyoyxho. eex h qqj csieh idp qfidt exiodeymi pgodaebgm... ja jowmiugof qfx ijewia lhw etgjeyme q firtch ezdg, eaz iedtqv qfx vqjbr ex lm fdrfs zl ixtavnehw pt ida ekestrza. p wepd ele dbq, a fiv mpgse rcevtglm p sjsl tracwda pke meoieyme-xd. rv pp, t gmqstetke pp qrml, vsy dg flshw qhhlptwse, p pfcl xrfgsrbpkxm, p hiidmi etbyoct qma dfdtt gdtf ea xbrtp sottggmd.";
    // let content = fs::read_to_string("./desafio1.txt").expect("file to be readable");

    
    let result = challenge(letter_freq_eng, desafio);
    // let ciphred_text = cipher("testandoessetrabalhoincrivel", "teste");
    // let deciphred_text = decipher(desafio, "arara");
    // println!("{}", deciphred_text);
}
