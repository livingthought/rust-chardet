extern crate chardet;
use std::fs::OpenOptions;
use std::io::prelude::*;

const TEST_TABLE:&[(&str, &str, f32, &str)] = &[
    ("tests/data/ascii/howto.diveintomark.org.xml","ascii", 0.99, ""),
    ("tests/data/ascii/_chromium_iso-8859-1_with_no_encoding_specified.html","ascii", 0.99, ""),
    ("tests/data/ascii/_mozilla_bug638318_text.html","ascii", 0.99, ""),
    ("tests/data/Big5/0804.blogspot.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/blog.worren.net.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/carbonxiv.blogspot.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/catshadow.blogspot.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/coolloud.org.tw.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/digitalwall.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/ebao.us.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/fudesign.blogspot.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/kafkatseng.blogspot.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/ke207.blogspot.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/leavesth.blogspot.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/letterlego.blogspot.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/linyijen.blogspot.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/marilynwu.blogspot.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/myblog.pchome.com.tw.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/oui-design.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/sanwenji.blogspot.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/sinica.edu.tw.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/sylvia1976.blogspot.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/tlkkuo.blogspot.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/unoriginalblog.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/upsaid.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/willythecop.blogspot.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/ytc.blogspot.com.xml","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/_chromium_Big5_with_no_encoding_specified.html","Big5", 0.99, "Chinese"),
    ("tests/data/Big5/_ude_1.txt","Big5", 0.99, "Chinese"),
    ("tests/data/CP932/hardsoft.at.webry.info.xml","CP932", 0.99, "Japanese"),
    ("tests/data/CP932/www2.chuo-u.ac.jp-suishin.xml","CP932", 0.99, "Japanese"),
    ("tests/data/CP932/y-moto.com.xml","CP932", 0.99, "Japanese"),
    ("tests/data/CP949/ricanet.com.xml","CP949", 0.99, "Korean"),
    ("tests/data/EUC-JP/aivy.co.jp.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/akaname.main.jp.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/arclamp.jp.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/aristrist.s57.xrea.com.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/artifact-jp.com.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/atom.ycf.nanet.co.jp.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/azito.under.jp.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/azoz.org.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/blog.kabu-navi.com.atom.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/blog.kabu-navi.com.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/bphrs.net.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/ch.kitaguni.tv.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/club.h14m.org.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/contents-factory.com.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/furusatonoeki.cutegirl.jp.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/manana.moo.jp.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/mimizun.com.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/misuzilla.org.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/overcube.com.atom.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/overcube.com.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/pinkupa.com.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/rdf.ycf.nanet.co.jp.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/siesta.co.jp.aozora.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/tls.org.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/yukiboh.moo.jp.xml","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/_mozilla_bug426271_text-euc-jp.html","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/_mozilla_bug431054_text.html","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/_mozilla_bug620106_text.html","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-JP/_ude_1.txt","EUC-JP", 0.99, "Japanese"),
    ("tests/data/EUC-KR/acnnewswire.net.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/alogblog.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/arts.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/birder.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/blog.bd-lab.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/blog.empas.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/blog.rss.naver.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/calmguy.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/chisato.info.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/console.linuxstudy.pe.kr.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/critique.or.kr.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/epitaph.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/ittrend.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/jely.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/jely.pe.kr.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/jowchung.oolim.net.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/kina.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/lennon81.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/oroll.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/poliplus.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/scarletkh2.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/siwoo.org.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/sparcs.kaist.ac.kr.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/tori02.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/willis.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/xenix.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/yunho.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/zangsalang.egloos.com.xml","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/_chromium_windows-949_with_no_encoding_specified.html","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/_mozilla_bug9357_text.html","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/_ude_euc1.txt","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-KR/_ude_euc2.txt","EUC-KR", 0.99, "Korean"),
    ("tests/data/EUC-TW/_ude_euc-tw1.txt","EUC-KR", 0.99, "Korean"),
    ("tests/data/GB2312/14.blog.westca.com.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/2.blog.westca.com.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/acnnewswire.net.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/bbs.blogsome.com.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/cappuccinos.3322.org.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/chen56.blogcn.com.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/cindychen.com.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/cnblog.org.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/coverer.com.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/eighthday.blogspot.com.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/godthink.blogsome.com.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/jjgod.3322.org.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/lily.blogsome.com.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/luciferwang.blogcn.com.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/pda.blogsome.com.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/softsea.net.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/w3cn.org.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/xy15400.blogcn.com.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/_chromium_gb18030_with_no_encoding_specified.html.xml","GB2312", 0.99, "Chinese"),
    ("tests/data/GB2312/_mozilla_bug171813_text.html","GB2312", 0.98, "Chinese"),
    ("tests/data/IBM855/aif.ru.health.xml","IBM855", 0.98, "Russian"),
    ("tests/data/IBM855/aug32.hole.ru.xml","IBM855", 0.78, "Russian"),
    ("tests/data/IBM855/aviaport.ru.xml","IBM855", 0.96, "Russian"),
    ("tests/data/IBM855/blog.mlmaster.com.xml","IBM855", 0.96, "Russian"),
    ("tests/data/IBM855/forum.template-toolkit.ru.1.xml","IBM855", 0.99, "Russian"),
    ("tests/data/IBM855/forum.template-toolkit.ru.4.xml","IBM855", 0.97, "Russian"),
    ("tests/data/IBM855/forum.template-toolkit.ru.6.xml","IBM855", 0.98, "Russian"),
    ("tests/data/IBM855/forum.template-toolkit.ru.8.xml","IBM855", 0.98, "Russian"),
    ("tests/data/IBM855/forum.template-toolkit.ru.9.xml","IBM855", 0.99, "Russian"),
    ("tests/data/IBM855/greek.ru.xml","IBM855", 0.87, "Russian"),
    ("tests/data/IBM855/intertat.ru.xml","IBM855", 0.94, "Russian"),
    ("tests/data/IBM855/janulalife.blogspot.com.xml","IBM855", 0.99, "Russian"),
    ("tests/data/IBM855/kapranoff.ru.xml","IBM855", 0.95, "Russian"),
    ("tests/data/IBM855/money.rin.ru.xml","IBM855", 0.96, "Russian"),
    ("tests/data/IBM855/music.peeps.ru.xml","IBM855", 0.96, "Russian"),
    ("tests/data/IBM855/newsru.com.xml","IBM855", 0.97, "Russian"),
    ("tests/data/IBM855/susu.ac.ru.xml","IBM855", 0.96, "Russian"),
    ("tests/data/IBM855/_ude_1.txt","IBM855", 0.95, "Russian"),
    ("tests/data/IBM866/aif.ru.health.xml","IBM866", 0.98, "Russian"),
    ("tests/data/IBM866/aug32.hole.ru.xml","IBM866", 0.78, "Russian"),
    ("tests/data/IBM866/aviaport.ru.xml","IBM866", 0.96, "Russian"),
    ("tests/data/IBM866/blog.mlmaster.com.xml","IBM866", 0.96, "Russian"),
    ("tests/data/IBM866/forum.template-toolkit.ru.1.xml","IBM866", 0.99, "Russian"),
    ("tests/data/IBM866/forum.template-toolkit.ru.4.xml","IBM866", 0.97, "Russian"),
    ("tests/data/IBM866/forum.template-toolkit.ru.6.xml","IBM866", 0.98, "Russian"),
    ("tests/data/IBM866/forum.template-toolkit.ru.8.xml","IBM866", 0.98, "Russian"),
    ("tests/data/IBM866/forum.template-toolkit.ru.9.xml","IBM866", 0.99, "Russian"),
    ("tests/data/IBM866/greek.ru.xml","IBM866", 0.89, "Russian"),
    ("tests/data/IBM866/intertat.ru.xml","IBM866", 0.94, "Russian"),
    ("tests/data/IBM866/janulalife.blogspot.com.xml","IBM866", 0.99, "Russian"),
    ("tests/data/IBM866/kapranoff.ru.xml","IBM866", 0.95, "Russian"),
    ("tests/data/IBM866/money.rin.ru.xml","IBM866", 0.96, "Russian"),
    ("tests/data/IBM866/music.peeps.ru.xml","IBM866", 0.96, "Russian"),
    ("tests/data/IBM866/newsru.com.xml","IBM866", 0.97, "Russian"),
    ("tests/data/IBM866/susu.ac.ru.xml","IBM866", 0.96, "Russian"),
    ("tests/data/IBM866/_ude_1.txt","IBM866", 0.95, "Russian"),
    ("tests/data/iso-2022-jp/_ude_1.txt","ISO-2022-JP", 0.99, "Japanese"),
    ("tests/data/iso-2022-kr/_ude_iso1.txt","ISO-2022-KR", 0.99, "Korean"),
    ("tests/data/iso-2022-kr/_ude_iso2.txt","ISO-2022-KR", 0.99, "Korean"),
    ("tests/data/iso-8859-1/_ude_1.txt","ISO-8859-1", 0.73, ""),
    ("tests/data/iso-8859-1/_ude_2.txt","ISO-8859-1", 0.73, ""),
    ("tests/data/iso-8859-1/_ude_3.txt","ISO-8859-1", 0.73, ""),
    ("tests/data/iso-8859-1/_ude_4.txt","ISO-8859-1", 0.73, ""),
    ("tests/data/iso-8859-1/_ude_5.txt","ISO-8859-1", 0.73, ""),
    ("tests/data/iso-8859-1/_ude_6.txt","ISO-8859-1", 0.62, ""),
    ("tests/data/iso-8859-2-hungarian/auto-apro.hu.xml","ISO-8859-1", 0.72, ""),
    ("tests/data/iso-8859-2-hungarian/cigartower.hu.xml","ISO-8859-1", 0.72, ""),
    ("tests/data/iso-8859-2-hungarian/escience.hu.xml","ISO-8859-1", 0.72, ""),
    ("tests/data/iso-8859-2-hungarian/hirtv.hu.xml","ISO-8859-1", 0.73, ""),
    ("tests/data/iso-8859-2-hungarian/honositomuhely.hu.xml","ISO-8859-1", 0.73, ""),
    ("tests/data/iso-8859-2-hungarian/saraspatak.hu.xml","ISO-8859-1", 0.73, ""),
    ("tests/data/iso-8859-2-hungarian/shamalt.uw.hu.mk.xml","ISO-8859-1", 0.73, ""),
    ("tests/data/iso-8859-2-hungarian/shamalt.uw.hu.mr.xml","ISO-8859-1", 0.73, ""),
    ("tests/data/iso-8859-2-hungarian/shamalt.uw.hu.mv.xml","ISO-8859-1", 0.73, ""),
    ("tests/data/iso-8859-2-hungarian/shamalt.uw.hu.xml","ISO-8859-1", 0.72, ""),
    ("tests/data/iso-8859-2-hungarian/ugyanmar.blogspot.com.xml","ISO-8859-1", 0.73, ""),
    ("tests/data/iso-8859-5-bulgarian/aero-bg.com.xml","ISO-8859-5", 0.99, "Bulgarian"),
    ("tests/data/iso-8859-5-bulgarian/bbc.co.uk.popshow.xml","ISO-8859-5", 0.99, "Bulgarian"),
    ("tests/data/iso-8859-5-bulgarian/bpm.cult.bg.2.xml","ISO-8859-5", 0.95, "Bulgarian"),
    ("tests/data/iso-8859-5-bulgarian/bpm.cult.bg.4.xml","ISO-8859-5", 0.99, "Bulgarian"),
    ("tests/data/iso-8859-5-bulgarian/bpm.cult.bg.9.xml","ISO-8859-5", 0.99, "Bulgarian"),
    ("tests/data/iso-8859-5-bulgarian/bpm.cult.bg.medusa.4.xml","ISO-8859-5", 0.99, "Bulgarian"),
    ("tests/data/iso-8859-5-bulgarian/bpm.cult.bg.xml","ISO-8859-5", 0.99, "Bulgarian"),
    ("tests/data/iso-8859-5-bulgarian/debian.gabrovo.com.news.xml","ISO-8859-5", 0.99, "Bulgarian"),
    ("tests/data/iso-8859-5-bulgarian/debian.gabrovo.com.xml","ISO-8859-5", 0.99, "Bulgarian"),
    ("tests/data/iso-8859-5-bulgarian/doncho.net.comments.xml","ISO-8859-5", 0.99, "Bulgarian"),
    ("tests/data/iso-8859-5-bulgarian/ecloga.cult.bg.xml","ISO-8859-5", 0.99, "Bulgarian"),
    ("tests/data/iso-8859-5-bulgarian/ide.li.xml","ISO-8859-5", 0.99, "Bulgarian"),
    ("tests/data/iso-8859-5-bulgarian/linux-bg.org.xml","ISO-8859-5", 0.97, "Bulgarian"),
    ("tests/data/iso-8859-5-cyrillic/aif.ru.health.xml","ISO-8859-5", 0.98, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/aug32.hole.ru.xml","ISO-8859-5", 0.78, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/aviaport.ru.xml","ISO-8859-5", 0.96, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/blog.mlmaster.com.xml","ISO-8859-5", 0.96, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/forum.template-toolkit.ru.1.xml","ISO-8859-5", 0.99, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/forum.template-toolkit.ru.4.xml","ISO-8859-5", 0.97, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/forum.template-toolkit.ru.6.xml","ISO-8859-5", 0.98, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/forum.template-toolkit.ru.8.xml","ISO-8859-5", 0.98, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/forum.template-toolkit.ru.9.xml","ISO-8859-5", 0.99, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/greek.ru.xml","ISO-8859-5", 0.88, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/intertat.ru.xml","ISO-8859-5", 0.94, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/janulalife.blogspot.com.xml","ISO-8859-5", 0.99, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/kapranoff.ru.xml","ISO-8859-5", 0.95, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/money.rin.ru.xml","ISO-8859-5", 0.96, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/music.peeps.ru.xml","ISO-8859-5", 0.96, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/newsru.com.xml","ISO-8859-5", 0.97, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/susu.ac.ru.xml","ISO-8859-5", 0.96, "Russian"),
    ("tests/data/iso-8859-5-cyrillic/_chromium_ISO-8859-5_with_no_encoding_specified.html","ISO-8859-5", 0.96, "Russian"),
    ("tests/data/iso-8859-6-arabic/_chromium_ISO-8859-6_with_no_encoding_specified.html","MacCyrillic", 0.30, "Russian"),
    ("tests/data/iso-8859-7-greek/disabled.gr.xml","windows-1253", 0.94, "Greek"),
    ("tests/data/iso-8859-7-greek/hotstation.gr.xml","ISO-8859-7", 0.95, "Greek"),
    ("tests/data/iso-8859-7-greek/naftemporiki.gr.bus.xml","ISO-8859-7", 0.54, "Greek"),
    ("tests/data/iso-8859-7-greek/naftemporiki.gr.cmm.xml","ISO-8859-7", 0.54, "Greek"),
    ("tests/data/iso-8859-7-greek/naftemporiki.gr.fin.xml","ISO-8859-7", 0.59, "Greek"),
    ("tests/data/iso-8859-7-greek/naftemporiki.gr.mrk.xml","ISO-8859-7", 0.51, "Greek"),
    ("tests/data/iso-8859-7-greek/naftemporiki.gr.mrt.xml","ISO-8859-7", 0.63, "Greek"),
    ("tests/data/iso-8859-7-greek/naftemporiki.gr.spo.xml","ISO-8859-7", 0.50, "Greek"),
    ("tests/data/iso-8859-7-greek/naftemporiki.gr.wld.xml","ISO-8859-7", 0.58, "Greek"),
    ("tests/data/iso-8859-7-greek/_chromium_ISO-8859-7_with_no_encoding_specified.html","ISO-8859-7", 0.96, "Greek"),
    ("tests/data/iso-8859-7-greek/_ude_greek.txt","ISO-8859-7", 0.94, "Greek"),
    ("tests/data/iso-8859-9-turkish/divxplanet.com.xml","ISO-8859-1", 0.73, ""),
    ("tests/data/iso-8859-9-turkish/subtitle.srt","ISO-8859-1", 0.73, ""),
    ("tests/data/iso-8859-9-turkish/wikitop_tr_ISO-8859-9.txt","ISO-8859-1", 0.73, ""),
    ("tests/data/KOI8-R/aif.ru.health.xml","KOI8-R", 0.98, "Russian"),
    ("tests/data/KOI8-R/aug32.hole.ru.xml","KOI8-R", 0.78, "Russian"),
    ("tests/data/KOI8-R/aviaport.ru.xml","KOI8-R", 0.96, "Russian"),
    ("tests/data/KOI8-R/blog.mlmaster.com.xml","KOI8-R", 0.96, "Russian"),
    ("tests/data/KOI8-R/forum.template-toolkit.ru.1.xml","KOI8-R", 0.99, "Russian"),
    ("tests/data/KOI8-R/forum.template-toolkit.ru.4.xml","KOI8-R", 0.97, "Russian"),
    ("tests/data/KOI8-R/forum.template-toolkit.ru.6.xml","KOI8-R", 0.98, "Russian"),
    ("tests/data/KOI8-R/forum.template-toolkit.ru.8.xml","KOI8-R", 0.98, "Russian"),
    ("tests/data/KOI8-R/forum.template-toolkit.ru.9.xml","KOI8-R", 0.99, "Russian"),
    ("tests/data/KOI8-R/greek.ru.xml","KOI8-R", 0.89, "Russian"),
    ("tests/data/KOI8-R/intertat.ru.xml","KOI8-R", 0.97, "Russian"),
    ("tests/data/KOI8-R/janulalife.blogspot.com.xml","KOI8-R", 0.99, "Russian"),
    ("tests/data/KOI8-R/kapranoff.ru.xml","KOI8-R", 0.95, "Russian"),
    ("tests/data/KOI8-R/koi.kinder.ru.xml","KOI8-R", 0.96, "Russian"),
    ("tests/data/KOI8-R/money.rin.ru.xml","KOI8-R", 0.96, "Russian"),
    ("tests/data/KOI8-R/music.peeps.ru.xml","KOI8-R", 0.96, "Russian"),
    ("tests/data/KOI8-R/newsru.com.xml","KOI8-R", 0.97, "Russian"),
    ("tests/data/KOI8-R/susu.ac.ru.xml","KOI8-R", 0.95, "Russian"),
    ("tests/data/KOI8-R/_chromium_KOI8-R_with_no_encoding_specified.html","KOI8-R", 0.96, "Russian"),
    ("tests/data/KOI8-R/_ude_1.txt","KOI8-R", 0.95, "Russian"),
    ("tests/data/MacCyrillic/aif.ru.health.xml","MacCyrillic", 0.98, "Russian"),
    ("tests/data/MacCyrillic/aug32.hole.ru.xml","MacCyrillic", 0.78, "Russian"),
    ("tests/data/MacCyrillic/aviaport.ru.xml","MacCyrillic", 0.96, "Russian"),
    ("tests/data/MacCyrillic/blog.mlmaster.com.xml","MacCyrillic", 0.96, "Russian"),
    ("tests/data/MacCyrillic/forum.template-toolkit.ru.4.xml","MacCyrillic", 0.97, "Russian"),
    ("tests/data/MacCyrillic/forum.template-toolkit.ru.6.xml","MacCyrillic", 0.98, "Russian"),
    ("tests/data/MacCyrillic/forum.template-toolkit.ru.8.xml","MacCyrillic", 0.98, "Russian"),
    ("tests/data/MacCyrillic/forum.template-toolkit.ru.9.xml","MacCyrillic", 0.99, "Russian"),
    ("tests/data/MacCyrillic/greek.ru.xml","MacCyrillic", 0.88, "Russian"),
    ("tests/data/MacCyrillic/intertat.ru.xml","MacCyrillic", 0.94, "Russian"),
    ("tests/data/MacCyrillic/kapranoff.ru.xml","MacCyrillic", 0.95, "Russian"),
    ("tests/data/MacCyrillic/koi.kinder.ru.xml","MacCyrillic", 0.95, "Russian"),
    ("tests/data/MacCyrillic/money.rin.ru.xml","MacCyrillic", 0.96, "Russian"),
    ("tests/data/MacCyrillic/music.peeps.ru.xml","MacCyrillic", 0.96, "Russian"),
    ("tests/data/MacCyrillic/newsru.com.xml","MacCyrillic", 0.97, "Russian"),
    ("tests/data/MacCyrillic/susu.ac.ru.xml","MacCyrillic", 0.96, "Russian"),
    ("tests/data/MacCyrillic/_ude_1.txt","MacCyrillic", 0.95, "Russian"),
    ("tests/data/SHIFT_JIS/10e.org.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/1affliate.com.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/accessories-brand.com.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/amefoot.net.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/andore.com.inami.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/andore.com.money.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/andore.com.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/blog.inkase.net.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/blog.paseri.ne.jp.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/bloglelife.com.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/brag.zaka.to.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/celeb.lalalu.com.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/clickablewords.com.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/do.beginnersrack.com.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/dogsinn.jp.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/grebeweb.net.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/milliontimes.jp.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/moon-light.ne.jp.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/nextbeaut.com.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/ooganemochi.com.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/perth-on.net.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/sakusaka-silk.net.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/setsuzei119.jp.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/tamuyou.haun.org.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/yasuhisa.com.xml","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/_chromium_Shift-JIS_with_no_encoding_specified.html","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/_ude_1.txt","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/_ude_2.txt","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/_ude_3.txt","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/SHIFT_JIS/_ude_4.txt","SHIFT_JIS", 0.99, "Japanese"),
    ("tests/data/TIS-620/opentle.org.xml","TIS-620", 0.77, "Thai"),
    ("tests/data/TIS-620/pharmacy.kku.ac.th.analyse1.xml","TIS-620", 0.80, "Thai"),
    ("tests/data/TIS-620/pharmacy.kku.ac.th.centerlab.xml","TIS-620", 0.84, "Thai"),
    ("tests/data/TIS-620/pharmacy.kku.ac.th.healthinfo-ne.xml","TIS-620", 0.75, "Thai"),
    ("tests/data/TIS-620/trickspot.boxchart.com.xml","TIS-620", 0.77, "Thai"),
    ("tests/data/TIS-620/_mozilla_bug488426_text.html","TIS-620", 0.76, "Thai"),
    ("tests/data/UTF-16/bom-utf-16-be.srt","UTF-16BE", 0.99, ""),
    ("tests/data/UTF-16/bom-utf-16-le.srt","UTF-16LE", 0.99, ""),
    ("tests/data/UTF-32/bom-utf-32-be.srt","UTF-32BE", 0.99, ""),
    ("tests/data/UTF-32/bom-utf-32-le.srt","UTF-32LE", 0.99, ""),
    ("tests/data/utf-8/anitabee.blogspot.com.xml","utf-8", 0.99, ""),
    ("tests/data/utf-8/balatonblog.typepad.com.xml","utf-8", 0.99, ""),
    ("tests/data/utf-8/boobooo.blogspot.com.xml","utf-8", 0.99, ""),
    ("tests/data/utf-8/linuxbox.hu.xml","utf-8", 0.99, ""),
    ("tests/data/utf-8/pihgy.hu.xml","utf-8", 0.99, ""),
    ("tests/data/utf-8/weblabor.hu.2.xml","utf-8", 0.99, ""),
    ("tests/data/utf-8/weblabor.hu.xml","utf-8", 0.99, ""),
    ("tests/data/utf-8/_chromium_UTF-8_with_no_encoding_specified.html","utf-8", 0.99, ""),
    ("tests/data/utf-8/_mozilla_bug306272_text.html","utf-8", 0.75, ""),
    ("tests/data/utf-8/_mozilla_bug426271_text-utf-8.html","utf-8", 0.99, ""),
    ("tests/data/utf-8/_ude_1.txt","utf-8", 0.99, ""),
    ("tests/data/utf-8/_ude_2.txt","utf-8", 0.99, ""),
    ("tests/data/utf-8/_ude_3.txt","utf-8", 0.87, ""),
    ("tests/data/utf-8/_ude_5.txt","utf-8", 0.99, ""),
    ("tests/data/utf-8/_ude_greek.txt","utf-8", 0.99, ""),
    ("tests/data/utf-8/_ude_he1.txt","utf-8", 0.99, ""),
    ("tests/data/utf-8/_ude_he2.txt","utf-8", 0.99, ""),
    ("tests/data/utf-8/_ude_he3.txt","utf-8", 0.99, ""),
    ("tests/data/utf-8/_ude_russian.txt","utf-8", 0.99, ""),
    ("tests/data/utf-8-sig/bom-utf-8.srt","UTF-8", 0.99, ""),
    ("tests/data/utf-8-sig/_ude_4.txt","UTF-8", 0.99, ""),
    ("tests/data/windows-1250-hungarian/bbc.co.uk.hu.forum.xml","ISO-8859-1", 0.73, ""),
    ("tests/data/windows-1250-hungarian/bbc.co.uk.hu.learningenglish.xml","ISO-8859-1", 0.73, ""),
    ("tests/data/windows-1250-hungarian/bbc.co.uk.hu.pressreview.xml","ISO-8859-1", 0.73, ""),
    ("tests/data/windows-1250-hungarian/bbc.co.uk.hu.xml","ISO-8859-1", 0.72, ""),
    ("tests/data/windows-1250-hungarian/objektivhir.hu.xml","ISO-8859-1", 0.72, ""),
    ("tests/data/windows-1250-hungarian/torokorszag.blogspot.com.xml","ISO-8859-1", 0.72, ""),
    ("tests/data/windows-1251-bulgarian/bbc.co.uk.popshow.xml","windows-1251", 0.99, "Bulgarian"),
    ("tests/data/windows-1251-bulgarian/bpm.cult.bg.2.xml","windows-1251", 0.95, "Bulgarian"),
    ("tests/data/windows-1251-bulgarian/bpm.cult.bg.3.xml","windows-1251", 0.99, "Bulgarian"),
    ("tests/data/windows-1251-bulgarian/bpm.cult.bg.4.xml","windows-1251", 0.99, "Bulgarian"),
    ("tests/data/windows-1251-bulgarian/bpm.cult.bg.9.xml","windows-1251", 0.99, "Bulgarian"),
    ("tests/data/windows-1251-bulgarian/bpm.cult.bg.medusa.4.xml","windows-1251", 0.99, "Bulgarian"),
    ("tests/data/windows-1251-bulgarian/bpm.cult.bg.xml","windows-1251", 0.98, "Bulgarian"),
    ("tests/data/windows-1251-bulgarian/debian.gabrovo.com.news.xml","windows-1251", 0.99, "Bulgarian"),
    ("tests/data/windows-1251-bulgarian/debian.gabrovo.com.xml","windows-1251", 0.99, "Bulgarian"),
    ("tests/data/windows-1251-bulgarian/doncho.net.comments.xml","windows-1251", 0.99, "Bulgarian"),
    ("tests/data/windows-1251-bulgarian/doncho.net.xml","windows-1251", 0.98, "Bulgarian"),
    ("tests/data/windows-1251-bulgarian/ecloga.cult.bg.xml","windows-1251", 0.99, "Bulgarian"),
    ("tests/data/windows-1251-bulgarian/ide.li.xml","windows-1251", 0.99, "Bulgarian"),
    ("tests/data/windows-1251-bulgarian/informator.org.xml","windows-1251", 0.82, "Bulgarian"),
    ("tests/data/windows-1251-bulgarian/linux-bg.org.xml","windows-1251", 0.97, "Bulgarian"),
    ("tests/data/windows-1251-bulgarian/rinennor.org.xml","windows-1251", 0.98, "Bulgarian"),
    ("tests/data/windows-1251-cyrillic/aif.ru.health.xml","windows-1251", 0.98, "Russian"),
    ("tests/data/windows-1251-cyrillic/anthropology.ru.xml","windows-1251", 0.92, "Russian"),
    ("tests/data/windows-1251-cyrillic/aug32.hole.ru.xml","windows-1251", 0.78, "Russian"),
    ("tests/data/windows-1251-cyrillic/aviaport.ru.xml","windows-1251", 0.96, "Russian"),
    ("tests/data/windows-1251-cyrillic/blog.mlmaster.com.xml","windows-1251", 0.96, "Russian"),
    ("tests/data/windows-1251-cyrillic/forum.template-toolkit.ru.1.xml","windows-1251", 0.99, "Russian"),
    ("tests/data/windows-1251-cyrillic/forum.template-toolkit.ru.4.xml","windows-1251", 0.97, "Russian"),
    ("tests/data/windows-1251-cyrillic/forum.template-toolkit.ru.6.xml","windows-1251", 0.98, "Russian"),
    ("tests/data/windows-1251-cyrillic/forum.template-toolkit.ru.8.xml","windows-1251", 0.98, "Russian"),
    ("tests/data/windows-1251-cyrillic/forum.template-toolkit.ru.9.xml","windows-1251", 0.99, "Russian"),
    ("tests/data/windows-1251-cyrillic/greek.ru.xml","windows-1251", 0.88, "Russian"),
    ("tests/data/windows-1251-cyrillic/intertat.ru.xml","windows-1251", 0.94, "Bulgarian"),
    ("tests/data/windows-1251-cyrillic/janulalife.blogspot.com.xml","windows-1251", 0.99, "Russian"),
    ("tests/data/windows-1251-cyrillic/kapranoff.ru.xml","windows-1251", 0.95, "Russian"),
    ("tests/data/windows-1251-cyrillic/money.rin.ru.xml","windows-1251", 0.96, "Russian"),
    ("tests/data/windows-1251-cyrillic/music.peeps.ru.xml","windows-1251", 0.96, "Russian"),
    ("tests/data/windows-1251-cyrillic/newsru.com.xml","windows-1251", 0.97, "Russian"),
    ("tests/data/windows-1251-cyrillic/_chromium_windows-1251_with_no_encoding_specified.html","windows-1251", 0.96, "Russian"),
    ("tests/data/windows-1251-cyrillic/_ude_1.txt","windows-1251", 0.95, "Russian"),
    ("tests/data/windows-1252/github_bug_9.txt","ISO-8859-1", 0.73, ""),
    ("tests/data/windows-1252/_mozilla_bug421271_text.html","ISO-8859-1", 0.73, ""),
    ("tests/data/windows-1254-turkish/_chromium_windows-1254_with_no_encoding_specified.html","ISO-8859-1", 0.73, ""),
    ("tests/data/windows-1255-hebrew/carshops.co.il.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/exego.net.2.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/hagada.org.il.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/halemo.net.edoar.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/hevra.org.il.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/hydepark.hevre.co.il.7957.xml","windows-1255", 0.96, "Hebrew"),
    ("tests/data/windows-1255-hebrew/info.org.il.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/infomed.co.il.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/law.co.il.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/maakav.org.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/neviim.net.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/notes.co.il.50.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/notes.co.il.6.xml","windows-1255", 0.97, "Hebrew"),
    ("tests/data/windows-1255-hebrew/notes.co.il.7.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/notes.co.il.8.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/pcplus.co.il.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/sharks.co.il.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/whatsup.org.il.xml","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/_chromium_ISO-8859-8_with_no_encoding_specified.html","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/_chromium_windows-1255_with_no_encoding_specified.html","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/_ude_he1.txt","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/_ude_he2.txt","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1255-hebrew/_ude_he3.txt","windows-1255", 0.99, "Hebrew"),
    ("tests/data/windows-1256-arabic/_chromium_windows-1256_with_no_encoding_specified.html","MacCyrillic", 0.26, "Russian"),
];
#[test]
fn basic_test() {
    for item in TEST_TABLE {
        let mut fh = OpenOptions::new().read(true).open(item.0).expect(
            "Could not open file",
        );
        let mut reader: Vec<u8> = Vec::new();
        fh.read_to_end(&mut reader).expect("Could not read file");
        let result = chardet::detect(&reader);
        println!("{:?}", item);
        println!("{:?}", result);
        assert!(result.0 == item.1, format!("Encoding Expected:[{}] Result:[{}]", item.1, result.0));
        assert!(result.1 >= item.2, format!("Confidence Expected:[{}] Result:[{}]", item.2, result.1));
        assert!(result.2 == item.3, format!("Language Expected:[{}] Result:[{}]", item.3, result.2));
    }
}

#[test]
fn reuse_test() {
    let mut detector = chardet::UniversalDetector::new();
    for item in TEST_TABLE {
        let mut fh = OpenOptions::new().read(true).open(item.0).expect(
            "Could not open file",
        );
        let mut reader: Vec<u8> = Vec::new();
        fh.read_to_end(&mut reader).expect("Could not read file");
        detector.feed(&reader);
        let result = detector.close();
        println!("{:?}", item);
        println!("{:?}", result);
        assert!(result.0 == item.1, format!("Encoding Expected:[{}] Result:[{}]", item.1, result.0));
        assert!(result.1 >= item.2, format!("Confidence Expected:[{}] Result:[{}]", item.2, result.1));
        assert!(result.2 == item.3, format!("Language Expected:[{}] Result:[{}]", item.3, result.2));
        detector.reset();
    }
}

#[test]
fn translate_test() {
    let char2enc:&[(String, &str)] = &[
        // different names in encoding
        ("CP932".to_string(), "windows-31j"),
        ("CP949".to_string(), "windows-949"),
        ("MacCyrillic".to_string(), "x-mac-cyrillic"),
        // not available in encoding
        ("IBM855".to_string(), "IBM855"),
        ("UTF-32BE".to_string(), "UTF-32BE"),
        ("UTF-32LE".to_string(), "UTF-32LE"),
    ];
    for item in char2enc {
        let enc = chardet::charset2encoding(&item.0);
        assert!( enc == item.1, format!("Charset: [{}] Expected Encoding: [{}] Result: [{}]", item.0, item.1, enc));
    }
}
