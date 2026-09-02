#!/usr/bin/env python3
"""Extract libxc `ext_params` defaults for each functional.

The kernels take their per-functional constants as ordinary `f64` arguments
(`param_kappa`, `param_mu`, ...). libxc keeps the defaults in C:

    #define PBE_N_PAR 2
    static const char  *pbe_names[PBE_N_PAR]  = {"_kappa", "_mu"};
    static const double pbe_values[PBE_N_PAR] = {0.8040, 0.2195149727645171};
    const xc_func_info_type xc_func_info_gga_x_pbe = {
      ...
      {PBE_N_PAR, pbe_names, pbe_desc, pbe_values, set_ext_params_cpy},
    };

This reads those out and pairs them with the kernel's parameter list.

Safety rules -- a functional is emitted only if all hold, otherwise it is
reported as unresolved rather than guessed at:

  1. Its `xc_func_info_` block names both a `_names` and a `_values` array (or
     is resolved via the verified npar==0 table / _init parser).
  2. The setter is a supported copy/transform setter.
  3. Every value parses as a float literal or evaluates via a safe AST over
     known C macros and arithmetic operators.
  4. `mapping` matches the kernel's parameter set exactly.
     This catches any case where maple2c named things differently from the ext_params table.

Usage: python3 tools/translate_rayon/extract_params.py [--json OUT]
"""
from __future__ import annotations

import argparse
import ast
import json
import math
import operator
import re
import struct
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
LIBXC_SRC = REPO / "libxc-master" / "src"
KERNELS = REPO / "crates" / "kernels-rayon"

SAFE_OPERATORS = {
    ast.Add: operator.add,
    ast.Sub: operator.sub,
    ast.Mult: operator.mul,
    ast.Div: operator.truediv,
    ast.USub: operator.neg,
    ast.UAdd: operator.pos,
}

_NAMES_RE = re.compile(
    r"(?:static\s+)?const\s+char\s*\*\s*(\w+)\s*\[[^\]]*\]\s*=\s*\{(.*?)\}\s*;", re.S
)
_VALUES_RE = re.compile(
    r"(?:static\s+)?const\s+double\s+(\w+)\s*\[[^\]]*\]\s*=\s*\{(.*?)\}\s*;", re.S
)
_INFO_RE = re.compile(
    r"const\s+xc_func_info_type\s+xc_func_info_(\w+)\s*=\s*\{(.*?)\n\};", re.S
)
_EXT_RE = re.compile(r"\{\s*([0-9A-Za-z_+\- ]+)\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*(\w+)\s*\}")
_FLOAT_RE = re.compile(r"^[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?$")

# Manual defaults for functionals whose xc_func_info declares npar == 0 or special table defaults (D-05c)
NULL_SETTER_DEFAULTS = {
    "lda_x": {"param_alpha": "1.0"},
    "lda_k_tf": {"param_ax": "1.104950565705860002098832079519635692942"},
    "gga_x_lb": {"param_alpha": "1.0", "param_beta": "0.05", "param_gamma": "1.0"},
    "gga_x_mpbe": {"param_a": "0.157", "param_c1": "0.21951", "param_c2": "-0.015", "param_c3": "0.0"},
    "lda_c_1d_csc": {
        "param_para_0": "18.40", "param_para_1": "0.0", "param_para_2": "7.501", "param_para_3": "0.10185", "param_para_4": "0.012827",
        "param_para_5": "2.0", "param_para_6": "3.0", "param_para_7": "1.511", "param_para_8": "0.258", "param_para_9": "4.424",
        "param_ferro_0": "5.24", "param_ferro_1": "0.0", "param_ferro_2": "1.568", "param_ferro_3": "0.12856", "param_ferro_4": "0.003201",
        "param_ferro_5": "2.0", "param_ferro_6": "3.0", "param_ferro_7": "0.0538", "param_ferro_8": "1.56e-5", "param_ferro_9": "2.958",
    },
    "lda_c_2d_prm": {
        "param_N": "2.0", "param_c": repr(math.pi / (2.0 * (2.0 - 1.0) * 3.9274 * 3.9274))
    },
    "lda_c_xalpha": {"param_alpha": "0.5"},
    "lda_k_lp": {"param_ax": "1.142427709758666675644309251677891925671"},
}

# Parameter name aliases (D-05b, R-04, V4-03)
PARAM_ALIASES = {
    ("gga_c_pbe", "_B"): "param_BB",
    ("gga_c_pbe_vwn", "_B"): "param_BB",
    ("gga_x_rpbe", "_kappa"): "param_rpbe_kappa",
    ("gga_x_rpbe", "_mu"): "param_rpbe_mu",
    ("gga_c_lm", "_f"): "param_lm_f",
    ("lda_x_1d_exponential", "parambeta"): "param_beta",
    ("lda_x_1d_soft", "parambeta"): "param_beta",
    ("mgga_x_tb09", "c"): "param_c",
    ("mgga_c_vsxc", "_alpha_os"): "param_alpha_ab",
    ("lda_c_hl", "_r0"): "param_hl_r_0",
    ("lda_c_hl", "_r1"): "param_hl_r_1",
    ("lda_c_hl", "_c0"): "param_hl_c_0",
    ("lda_c_hl", "_c1"): "param_hl_c_1",
    ("gga_k_pg1", "_mu"): "param_pg_mu",
    ("hyb_mgga_x_js18", "_a"): "param_hyb_coeff_0",
}

# Systematic aliases for R-04 name mismatches:
# gga_k_lgap, gga_k_lgap_ge
for f in ("gga_k_lgap", "gga_k_lgap_ge"):
    PARAM_ALIASES[(f, "_mu1")] = "param_mu_0"
    PARAM_ALIASES[(f, "_mu2")] = "param_mu_1"
    PARAM_ALIASES[(f, "_mu3")] = "param_mu_2"

# gga_k_dk and siblings
for f in ("gga_k_dk", "gga_k_perdew", "gga_k_vsk", "gga_k_vjks", "gga_k_ernzerhof"):
    for i in range(5):
        PARAM_ALIASES[(f, f"_a{i}")] = f"param_aa_{i}"
        PARAM_ALIASES[(f, f"_b{i}")] = f"param_bb_{i}"

# lda_c_pz
for i in (0, 1):
    PARAM_ALIASES[("lda_c_pz", f"_beta1{i}")] = f"param_beta1_{i}"
    PARAM_ALIASES[("lda_c_pz", f"_beta2{i}")] = f"param_beta2_{i}"

# mgga_c_tpss, mgga_c_revtpss
for f in ("mgga_c_tpss", "mgga_c_revtpss"):
    for i in range(4):
        PARAM_ALIASES[(f, f"_C0_c{i}")] = f"param_C0_c_{i}"

# gga_c_bmk
for i in range(5):
    PARAM_ALIASES[("gga_c_bmk", f"_cos{i}")] = f"param_c_ab_{i}"
    PARAM_ALIASES[("gga_c_bmk", f"_css{i}")] = f"param_c_ss_{i}"

# gga_c_sogga11, gga_c_sogga11_x
for f in ("gga_c_sogga11", "gga_c_sogga11_x"):
    for i in range(6):
        PARAM_ALIASES[(f, f"_a{i}")] = f"param_sogga11_a_{i}"
        PARAM_ALIASES[(f, f"_b{i}")] = f"param_sogga11_b_{i}"

# mgga_x_task
PARAM_ALIASES[("mgga_x_task", "_c")] = "param_task_c"
PARAM_ALIASES[("mgga_x_task", "_d")] = "param_task_d"
PARAM_ALIASES[("mgga_x_task", "_h0x")] = "param_task_h0x"
for i in range(3):
    PARAM_ALIASES[("mgga_x_task", f"_anu{i}")] = f"param_task_anu_{i}"
for i in range(5):
    PARAM_ALIASES[("mgga_x_task", f"_bnu{i}")] = f"param_task_bnu_{i}"

# mgga_x_tau_hcth
for i in range(4):
    PARAM_ALIASES[("mgga_x_tau_hcth", f"_cxl{i}")] = f"param_cx_local_{i}"
    PARAM_ALIASES[("mgga_x_tau_hcth", f"_cxnl{i}")] = f"param_cx_nlocal_{i}"

# gga_xc_th1
for i in range(21):
    PARAM_ALIASES[("gga_xc_th1", f"_w[{i}]")] = f"param_omega_{i}"

# mgga_x_m06l
for f in ("mgga_x_m06l", "mgga_x_m06_l", "mgga_x_revm06_l"):
    for i in range(12):
        PARAM_ALIASES[(f, f"_a{i}")] = f"param_a_{i}"
    for i in range(6):
        PARAM_ALIASES[(f, f"_d{i}")] = f"param_d_{i}"

# hyb_mgga_x_m06_sx (V4-03c)
for i in range(12):
    PARAM_ALIASES[("hyb_mgga_x_m06_sx", f"_a{i}")] = f"param_a_{i}"
for i in range(6):
    PARAM_ALIASES[("hyb_mgga_x_m06_sx", f"_b{i}")] = f"param_d_{i}"
PARAM_ALIASES[("hyb_mgga_x_m06_sx", "b3")] = "param_d_3"

# s12 family (V4-03c)
for n in ("_A", "_B", "_C", "_D", "_E"):
    PARAM_ALIASES[("gga_x_s12g", n)] = f"param_{n.lstrip('_')}"

for b in ("hyb_gga_x_cam_s12g", "hyb_gga_x_cam_s12h"):
    PARAM_ALIASES[(b, "_alpha")] = "param_hyb_coeff_1"
    PARAM_ALIASES[(b, "_beta")] = "param_hyb_coeff_0"
    PARAM_ALIASES[(b, "_omega")] = "param_hyb_omega_0"

# b97 family + wb97 family (V4-03b)
for b in ("gga_xc_b97", "gga_xc_b97_d", "gga_xc_b97_3c", "gga_xc_b97_gga1", "gga_xc_hcth_93",
          "gga_xc_hcth_120", "gga_xc_hcth_147", "gga_xc_hcth_407", "gga_xc_hcth_407p",
          "gga_xc_hcth_p14", "gga_xc_hcth_p76", "gga_xc_hle16", "hyb_gga_xc_b97",
          "hyb_gga_xc_b97_1", "hyb_gga_xc_b97_1p", "hyb_gga_xc_b97_2", "hyb_gga_xc_b97_3",
          "hyb_gga_xc_b97_k", "hyb_gga_xc_sb98_1a", "hyb_gga_xc_sb98_1b", "hyb_gga_xc_sb98_1c",
          "hyb_gga_xc_sb98_2a", "hyb_gga_xc_sb98_2b", "hyb_gga_xc_sb98_2c",
          "hyb_gga_xc_wb97", "hyb_gga_xc_wb97x", "hyb_gga_xc_wb97x_d", "hyb_gga_xc_wb97x_d3", "hyb_gga_xc_wb97x_v"):
    for i in range(5):
        PARAM_ALIASES[(b, f"_cx{i}")] = f"param_c_x_{i}"
        PARAM_ALIASES[(b, f"_css{i}")] = f"param_c_ss_{i}"
        PARAM_ALIASES[(b, f"_cos{i}")] = f"param_c_ab_{i}"

# m08 correlation family
for b in ("mgga_c_m08", "mgga_c_m08_hx", "mgga_c_m08_so", "mgga_c_m11", "mgga_c_m11_l",
          "mgga_c_mn12_l", "mgga_c_mn12_sx", "mgga_c_mn15", "mgga_c_mn15_l", "mgga_c_revm11"):
    for i in range(12):
        PARAM_ALIASES[(b, f"_a{i}")] = f"param_m08_a_{i}"
        PARAM_ALIASES[(b, f"_b{i}")] = f"param_m08_b_{i}"

# csk family
for b in ("mgga_k_csk", "mgga_k_csk1", "mgga_k_csk2", "mgga_k_csk3", "mgga_k_csk4"):
    PARAM_ALIASES[(b, "_a")] = "param_csk_a"

for b in ("mgga_k_csk_loc", "mgga_k_csk_loc1", "mgga_k_csk_loc2", "mgga_k_csk_loc3", "mgga_k_csk_loc4"):
    PARAM_ALIASES[(b, "_a")] = "param_csk_a"
    PARAM_ALIASES[(b, "_cp")] = "param_csk_cp"
    PARAM_ALIASES[(b, "_cq")] = "param_csk_cq"

# pgsl family
for b in ("mgga_k_pgslb", "mgga_k_pgsl025", "mgga_k_pgsl050", "mgga_k_pgsl075", "mgga_k_pgsl100"):
    PARAM_ALIASES[(b, "_beta")] = "param_pgslb_beta"
    PARAM_ALIASES[(b, "_mu")] = "param_pgslb_mu"


# mgga_xc_b97m_v (V4-03b)
for i, n in enumerate(["_cx00", "_cx01", "_cx02", "_cx10", "_cx11"]):
    PARAM_ALIASES[("mgga_xc_b97m_v", n)] = f"param_c_x_{i}"
for i, n in enumerate(["_css00", "_css02", "_css10", "_css32", "_css42"]):
    PARAM_ALIASES[("mgga_xc_b97m_v", n)] = f"param_c_ss_{i}"
for i, n in enumerate(["_cos00", "_cos01", "_cos03", "_cos10", "_cos32"]):
    PARAM_ALIASES[("mgga_xc_b97m_v", n)] = f"param_c_os_{i}"

# b97m_v / gas22 family (V4-03b)
for i, n in enumerate(["_cx00", "_cx01", "_cx10"]):
    PARAM_ALIASES[("hyb_mgga_xc_wb97m_v", n)] = f"param_c_x_{i}"
    PARAM_ALIASES[("hyb_mgga_xc_gas22", n)] = f"param_c_x_{i}"
for i, n in enumerate(["_css00", "_css04", "_css10", "_css20", "_css43"]):
    PARAM_ALIASES[("hyb_mgga_xc_wb97m_v", n)] = f"param_c_ss_{i}"
    PARAM_ALIASES[("hyb_mgga_xc_gas22", n)] = f"param_c_ss_{i}"
for i, n in enumerate(["_cos00", "_cos10", "_cos20", "_cos21", "_cos60", "_cos61"]):
    PARAM_ALIASES[("hyb_mgga_xc_wb97m_v", n)] = f"param_c_os_{i}"
for i, n in enumerate(["_cos00", "_cos10", "_cos20", "_cos21", "_cos60"]):
    PARAM_ALIASES[("hyb_mgga_xc_gas22", n)] = f"param_c_os_{i}"

# mn12 exchange family
mn12_names = [
  "_CC000", "_CC001", "_CC002", "_CC003", "_CC004", "_CC005",
  "_CC010", "_CC011", "_CC012", "_CC013", "_CC014",
  "_CC020", "_CC021", "_CC022", "_CC023",
  "_CC030", "_CC031", "_CC032",
  "_CC100", "_CC101", "_CC102", "_CC103", "_CC104",
  "_CC110", "_CC111", "_CC112", "_CC113",
  "_CC120", "_CC121", "_CC122",
  "_CC200", "_CC201", "_CC202", "_CC203",
  "_CC210", "_CC211", "_CC212",
  "_CC300", "_CC301", "_CC302"
]
for b in ("mgga_x_mn12", "mgga_x_mn12_l", "mgga_x_mn12_sx", "mgga_x_mn15", "mgga_x_mn15_l"):
    for i, n in enumerate(mn12_names):
        PARAM_ALIASES[(b, n)] = f"param_c_{i}"

# KSDT table constants for lda_xc_ksdt family (V4-02)
KSDT_TABLES = {
    "lda_xc_ksdt": {
        "b": [[0.2839970, 48.9321540, 0.3709190, 61.0953570, 0.871837422702767684673873513724],
              [0.3290010, 111.5983080, 0.5370530, 105.0866630, 1.26233194679913807935662124247]],
        "c": [[0.8700890, 0.1930770, 2.4146440],
              [0.8489300, 0.1679520, 0.0888200]],
        "d": [[0.5798240, 94.5374540, 97.8396030, 59.9399990, 24.3880370],
              [0.5513300, 180.2131590, 134.4862310, 103.8616950, 17.7507100]],
        "e": [[0.2120360, 16.7312490, 28.4857920, 34.0288760, 17.2355150],
              [0.1531240, 19.5439450, 43.4003370, 120.2551450, 15.6628360]],
    },
    "lda_xc_corrksdt": {
        "b": [[0.342554, 9.141315, 0.448483, 18.553096, 1.05414999729322402165649834296],
              [0.3290010, 111.5983080, 0.5370530, 105.0866630, 1.26233194679913807935662124247]],
        "c": [[0.875130, -0.256320, 0.953988],
              [0.8489300, 0.1679520, 0.0888200]],
        "d": [[0.725917, 2.237347, 0.280748, 4.185911, 0.692183],
              [0.5513300, 180.2131590, 134.4862310, 103.8616950, 17.7507100]],
        "e": [[0.255415, 0.931933, 0.115398, 17.234117, 0.451437],
              [0.1531240, 19.5439450, 43.4003370, 120.2551450, 15.6628360]],
    },
    "lda_xc_gdsmfb": {
        "b": [[0.34369020, 7.82159531356, 0.300483986662, 15.8443467125, 0.70628138352268528131],
              [0.84987704, 3.04033012073, 0.0775730131248, 7.57703592489, 0.22972614201992673860]],
        "c": [[0.87594420, -0.2301308435510, 1.0],
              [0.91126873, -0.0307957123308, 1.0]],
        "d": [[0.72700876, 2.38264734144, 0.302212372510, 4.39347718395, 0.729951339845],
              [1.48658718, 4.92684905511, 0.0849387225179, 8.3269821188, 0.218864952126]],
        "e": [[0.25388214, 0.815795138599, 0.0646844410481, 15.0984620477, 0.230761357474],
              [0.27454097, 0.400994856555, 2.88773194962, 6.33499237092, 24.823008753]],
    },
}

# Setters supported with full verification (R-03, V4-02)
SUPPORTED_SETTERS = {
    "set_ext_params_cpy",
    "set_ext_params_cpy_omega",
    "set_ext_params_cpy_exx",
    "set_ext_params_cpy_cam",
    "set_ext_params_cpy_cam_sr",
    "set_ext_params_cpy_lc",
    "bn05_set_ext_params",
    "case21_set_ext_params",
    "lspbe_set_ext_params",
    "lsrpbe_set_ext_params",
    "N_set_ext_params",
    "T_set_ext_params",
    "pbe_lambda_set_ext_params",
    "s12h_set_ext_params",
    "lcgau_set_ext_params",
    "mpw91_set_ext_params",
}



# ---------------------------------------------------------------------------
# Runtime ext_params validation
# ---------------------------------------------------------------------------

_META_RS = REPO / "crates" / "libxc-core" / "src" / "meta" / "generated.rs"


def load_meta_ext_params() -> dict[str, list[tuple[str, str]]]:
    """`XC_<NAME>` -> [(ext_param name, default_value literal)], in libxc order.

    This is the array `libxc-eval` hands to `dispatch_with` at runtime, so it
    is the one the permutation has to be consistent with. Reading it here --
    rather than trusting that it agrees with the C source we just parsed --
    is what turns "runtime ext_params are wired" into a checked claim.
    """
    if not _META_RS.is_file():
        return {}
    src = _META_RS.read_text(errors="replace")
    out: dict[str, list[tuple[str, str]]] = {}
    for m in re.finditer(
            r"static (XC_[A-Z0-9_]+)_EXT_PARAMS: &\[ExtParamSpec\] = &\[(.*?)\];",
            src, re.S):
        out[m.group(1)] = [
            (n, v.strip()) for n, v in re.findall(
                r'ExtParamSpec \{ name: "([^"]*)", description: '
                r'"(?:[^"\\]|\\.)*", default_value: ([^,]+),', m.group(2))]
    return out


def _same_f64(a: str, b: str) -> bool:
    try:
        return struct.pack("<d", float(a)) == struct.pack("<d", float(b))
    except (ValueError, OverflowError):
        return False


def validate_ext_wiring(func, kp, values, ext_names, ext_to_kernel, meta_ext):
    """Return (ext_names, ext_to_kernel, reason_if_refused).

    A functional may accept runtime ext_params only if all three hold:

      1. a permutation was built at all;
      2. the metadata ext_param NAMES are exactly the libxc names, in order;
      3. every metadata DEFAULT lands, through the permutation, bit-for-bit on
         the kernel default it is supposed to feed.

    (3) is the real gate. It proves the permutation is value-consistent, which
    makes "pass the metadata defaults" a provable no-op -- the same bits the
    compiled-in constants already produce. Where it fails, the setter is doing
    something other than a copy: `gga_x_lspbe`'s does `mu += alpha*(1+kappa)`,
    so the metadata carries libxc's raw `_mu` while the kernel wants the
    transformed one. Feeding that raw value through would silently change the
    functional. Refuse instead, exactly as `UNSUPPORTED` does elsewhere.
    """
    if ext_to_kernel is None:
        return None, None, None
    key = "XC_" + func.upper()
    mp = meta_ext.get(key)
    if mp is None:
        return None, None, "no ext_params block in libxc-core metadata"
    if [n for n, _ in mp] != list(ext_names):
        return None, None, (
            f"metadata ext_param names {[n for n, _ in mp]} differ from libxc's "
            f"{list(ext_names)}")
    for i, (slot, (mn, mv)) in enumerate(zip(ext_to_kernel, mp)):
        if slot is None:
            continue
        if not _same_f64(mv, values[slot]):
            return None, None, (
                f"metadata default {mn}={mv} does not match the kernel default "
                f"{kp[slot]}={values[slot]}; the libxc setter transforms values "
                f"rather than copying them")
    return ext_names, ext_to_kernel, None


def strip_comments(src: str) -> str:
    src = re.sub(r"/\*.*?\*/", "", src, flags=re.S)
    return re.sub(r"//[^\n]*", "", src)


def load_defines() -> dict[str, str]:
    defines = {}
    for f in sorted(list(LIBXC_SRC.glob("*.h")) + list(LIBXC_SRC.glob("*.c"))):
        src = strip_comments(f.read_text(errors="replace"))
        for m in re.finditer(r"#\s*define\s+(\w+)\s+([^\n]+)", src):
            name, val = m.group(1), m.group(2).strip().split("/*")[0].split("//")[0].strip()
            defines[name] = val
    return defines


GLOBAL_DEFINES = load_defines()


def safe_eval_expr(expr_str: str, local_defines: dict[str, str] | None = None) -> str | None:
    expr_str = expr_str.strip()
    if _FLOAT_RE.match(expr_str):
        return expr_str

    s = re.sub(r"(?<=\d)[LlFf]\b", "", expr_str)
    if _FLOAT_RE.match(s):
        return s

    all_defs = dict(GLOBAL_DEFINES)
    if local_defines:
        all_defs.update(local_defines)

    for _ in range(10):
        changed = False
        for k, v in all_defs.items():
            if re.search(r"\b" + re.escape(k) + r"\b", s):
                s = re.sub(r"\b" + re.escape(k) + r"\b", f"({v})", s)
                changed = True
        s = re.sub(r"(?<=\d)[LlFf]\b", "", s)
        if not changed:
            break

    try:
        tree = ast.parse(s, mode="eval")

        def _eval(node):
            if isinstance(node, ast.Expression):
                return _eval(node.body)
            elif isinstance(node, ast.Constant):
                return float(node.value)
            elif isinstance(node, ast.UnaryOp) and type(node.op) in SAFE_OPERATORS:
                return SAFE_OPERATORS[type(node.op)](_eval(node.operand))
            elif isinstance(node, ast.BinOp) and type(node.op) in SAFE_OPERATORS:
                return SAFE_OPERATORS[type(node.op)](_eval(node.left), _eval(node.right))
            else:
                return None

        val = _eval(tree)
        if val is not None:
            return repr(val)
    except Exception:
        pass
    return None


def load_libxc() -> tuple[dict, dict, dict, dict]:
    """Return (names, values, infos, file_defines), all keyed per source file."""
    names, values, infos, file_defines = {}, {}, {}, {}
    for c in sorted(LIBXC_SRC.glob("*.c")):
        key = c.name
        src = strip_comments(c.read_text(errors="replace"))
        names[key] = {
            m.group(1): re.findall(r'"([^"]*)"', m.group(2))
            for m in _NAMES_RE.finditer(src)
        }
        values[key] = {
            m.group(1): [x.strip() for x in m.group(2).split(",") if x.strip()]
            for m in _VALUES_RE.finditer(src)
        }
        for m in _INFO_RE.finditer(src):
            infos[m.group(1)] = (key, m.group(2))
        f_defs = {}
        for m in re.finditer(r"#\s*define\s+(\w+)\s+([^\n]+)", src):
            name, val = m.group(1), m.group(2).strip().split("/*")[0].split("//")[0].strip()
            f_defs[name] = val
        file_defines[key] = f_defs
    return names, values, infos, file_defines


def kernel_params(fam: str, func: str) -> list[str] | None:
    d = KERNELS / fam / func / "src"
    for order in ("exc_unpol", "vxc_unpol", "fxc_unpol", "exc_pol", "vxc_pol"):
        f = d / f"{order}.rs"
        if not f.is_file():
            continue
        m = re.search(r"pub fn \w+\(\n(.*?)\n\) \{", f.read_text(), re.S)
        if not m:
            continue
        out = []
        for line in m.group(1).split("\n"):
            line = line.strip().rstrip(",")
            if not line or "&" in line:
                continue
            nm, _, ty = line.partition(":")
            nm, ty = nm.strip(), ty.strip()
            if ty == "f64" and nm not in ("dens_threshold", "zeta_threshold"):
                out.append(nm)
        return out
    return None


def resolve_init_defaults(func: str, fam: str, base: str, sfile: str, kp: list[str], infos: dict, file_defs: dict) -> tuple[dict | None, str | None]:
    """Parse defaults from _init function for npar == 0 functionals (V4-01)."""
    entry = infos.get(func)
    if not entry:
        return None, "no xc_func_info_ block in libxc source"
    _, info_str = entry

    # Locate init function name: `{0, NULL, NULL, NULL, NULL},\n  init_func, ...`
    init_m = re.search(r"\{\s*0\s*,\s*NULL\s*,\s*NULL\s*,\s*NULL\s*,\s*NULL\s*\}\s*,\s*(\w+)", info_str)
    if not init_m:
        return None, "no init function named in info block"
    init_fn = init_m.group(1)

    text = strip_comments((LIBXC_SRC / sfile).read_text(errors="replace"))
    fn_m = re.search(r"\b" + init_fn + r"\s*\([^)]*\)\s*\{(.*?)\n\}", text, re.S)
    if not fn_m:
        return None, f"init function {init_fn} body not found in {sfile}"
    fn_body = fn_m.group(1)

    if "xc_mix_init" in fn_body:
        return None, f"defaults are in {init_fn}, but functional is an auxiliary mix (xc_mix_init)"

    base_assignments = {}
    switch_part = ""
    if "switch" in fn_body:
        pre_switch, _, post_switch = fn_body.partition("switch")
        for assign in re.finditer(r"params\s*->\s*(\w+)\s*=\s*([^;]+);", pre_switch):
            base_assignments[assign.group(1)] = assign.group(2).strip()
        switch_part = "switch" + post_switch
    else:
        for assign in re.finditer(r"params\s*->\s*(\w+)\s*=\s*([^;]+);", fn_body):
            base_assignments[assign.group(1)] = assign.group(2).strip()

    case_assignments = dict(base_assignments)
    if switch_part:
        xc_macro = "XC_" + func.upper()
        case_m = re.search(r"case\s+" + re.escape(xc_macro) + r"\s*:(.*?)(?:break\s*;|case\s+XC_|\})", switch_part, re.S)
        if case_m:
            case_body = case_m.group(1)
            for assign in re.finditer(r"params\s*->\s*(\w+)\s*=\s*([^;]+);", case_body):
                case_assignments[assign.group(1)] = assign.group(2).strip()
        else:
            fallthrough_m = re.search(r"case\s+" + re.escape(xc_macro) + r"\s*:(?:(?!\bbreak\b).)*?case\s+\w+\s*:(.*?)(?:break\s*;|\})", switch_part, re.S)
            if fallthrough_m:
                case_body = fallthrough_m.group(1)
                for assign in re.finditer(r"params\s*->\s*(\w+)\s*=\s*([^;]+);", case_body):
                    case_assignments[assign.group(1)] = assign.group(2).strip()

    if "omega_TH3" in fn_body or "omega_TH4" in fn_body:
        arr_name = "omega_TH3" if "3" in func else "omega_TH4"
        arr_m = re.search(r"(?:static\s+)?const\s+double\s+" + arr_name + r"\s*\[[^\]]*\]\s*=\s*\{(.*?)\}\s*;", text, re.S)
        if not arr_m:
            arr_m = re.search(r"(?:static\s+)?double\s+" + arr_name + r"\s*\[[^\]]*\]\s*=\s*\{(.*?)\}\s*;", text, re.S)
        if arr_m:
            vals = [x.strip() for x in arr_m.group(1).split(",") if x.strip()]
            for i, v in enumerate(vals):
                case_assignments[f"omega_{i}"] = v

    if not case_assignments:
        return None, f"defaults are in {init_fn} and could not be scraped: no assignments found"

    mapping = {}
    f_defs = file_defs.get(sfile, {})
    for k, v in case_assignments.items():
        norm_k = f"param_{k}"
        if norm_k not in kp:
            if f"param_{k.lower()}" in kp:
                norm_k = f"param_{k.lower()}"
            elif f"param_{base}_{k}" in kp:
                norm_k = f"param_{base}_{k}"
            elif (func, f"_{k}") in PARAM_ALIASES:
                norm_k = PARAM_ALIASES[(func, f"_{k}")]
            elif (base, f"_{k}") in PARAM_ALIASES:
                norm_k = PARAM_ALIASES[(base, f"_{k}")]
            elif f"param_{k}" not in kp:
                for p in kp:
                    if p.endswith(f"_{k}") or p.endswith(f"_{k.lower()}"):
                        norm_k = p
                        break

        ev = safe_eval_expr(v, f_defs)
        if ev is None:
            return None, f"defaults are in {init_fn} and could not be scraped: expr {v} failed to evaluate"
        if norm_k in kp:
            mapping[norm_k] = ev

    if set(mapping) != set(kp):
        only_k = sorted(set(kp) - set(mapping))
        only_l = sorted(set(mapping) - set(kp))
        return None, f"defaults are in {init_fn} but param set mismatch: kernel-only={only_k} init-only={only_l}"

    return {
        "family": fam,
        "base_kernel": base,
        "params": kp,
        "values": [mapping[p] for p in kp],
        # npar == 0: the values were scraped from the _init function, so this
        # functional has no libxc ext_params array to accept at runtime.
        "ext_names": None,
        "ext_to_kernel": None,
    }, None


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--json", default=None)
    args = ap.parse_args()

    names, values, infos, file_defs = load_libxc()

    # R-02: Reverse map concrete functionals to (family, base_kernel, srcfile).
    #
    # One libxc .c file usually defines several functionals that genuinely do
    # share its maple2c kernel (`gga_x_pbe.c` defines pbe, pbe_sol, pbe_r … all
    # from one formula with different ext_params). But a file may *also* define
    # **composed** functionals -- built by `xc_mix_init` out of other
    # functionals -- which have no formula of their own. `gga_c_zvpbeloc.c` is
    # the clearest case: it defines `gga_c_zvpbeloc` from the maple kernel, and
    # `hyb_gga_xc_apbe0` and `hyb_gga_xc_hapbe` from `xc_mix_init`.
    #
    # Pairing every info block with the file's maple include therefore wires a
    # composed functional to an unrelated formula, and it evaluates silently and
    # wrongly -- `hyb_gga_xc_apbe0` was computing `gga_c_zvpbeloc` (oracle: 238x
    # relative error on vsigma). libxc marks the difference in the info block
    # itself: a kernel-backed functional carries a work pointer
    # (`NULL, &work_gga, NULL`), a composed one has an init function and no work
    # pointer at all (`xc_hyb_gga_xc_apbe0_init, NULL,  NULL, NULL, NULL`).
    # So require the work pointer, and report the rest rather than guessing.
    c_to_base: dict[str, tuple[str, str, str]] = {}
    composed: dict[str, str] = {}
    for c in sorted(LIBXC_SRC.glob("*.c")):
        text = c.read_text(errors="replace")
        incs = re.findall(r"#include\s+[\"<]maple2c/([^/\">]+)/([^/\">]+)\.c[\">]", text)
        if not incs:
            continue
        bases = {b for _, b in incs}
        if len(bases) > 1:
            # No file in libxc 7.0.0 does this. If one ever does, attributing
            # its infos to a single base is a guess, so stop rather than pick.
            raise SystemExit(
                f"{c.name} includes several maple2c kernels {sorted(bases)}; "
                "per-functional attribution is needed before it can be wired")
        fam = incs[0][0].split("_")[0]
        maple_base = incs[0][1]
        for m in re.finditer(
                r"const\s+xc_func_info_type\s+xc_func_info_(\w+)\s*=\s*\{(.*?)\n\};",
                text, re.S):
            info, body = m.group(1), m.group(2)
            if re.search(r"&\s*work_", body):
                c_to_base[info] = (fam, maple_base, c.name)
            else:
                composed[info] = c.name

    resolved, unresolved = {}, {}
    for info, srcfile in sorted(composed.items()):
        unresolved[info] = (
            f"composed functional: {srcfile} builds it with xc_mix_init out of "
            "other functionals, so it has no maple2c kernel of its own")

    meta_ext = load_meta_ext_params()
    ext_unwired: dict[str, str] = {}

    for func, (fam, base, srcfile) in sorted(c_to_base.items()):
        kp = kernel_params(fam, base)
        if kp is None:
            unresolved[func] = f"base kernel {base} not found in kernel tree"
            continue
        if not kp:
            resolved[func] = {"family": fam, "base_kernel": base, "params": [], "values": [],
                              "ext_names": [], "ext_to_kernel": []}
            continue

        if func in NULL_SETTER_DEFAULTS:
            mapping = NULL_SETTER_DEFAULTS[func]
            if set(mapping) == set(kp):
                resolved[func] = {
                    "family": fam,
                    "base_kernel": base,
                    "params": kp,
                    "values": [mapping[p] for p in kp],
                    # Defaults come from the functional's _init, not from an
                    # ext_params array, so there is no runtime order to honour.
                    "ext_names": None,
                    "ext_to_kernel": None,
                }
                continue

        entry = infos.get(func)
        if entry is None:
            unresolved[func] = "no xc_func_info_ block in libxc source"
            continue
        sfile, info = entry
        m = _EXT_RE.search(info)
        if not m:
            unresolved[func] = "no ext_params tuple in xc_func_info_"
            continue
        npar, nm_arr, _desc, val_arr, setter = [x.strip() for x in m.groups()]

        # V4-01: When npar == 0, resolve defaults from _init function
        if npar == "0":
            res, err = resolve_init_defaults(func, fam, base, sfile, kp, infos, file_defs)
            if res:
                resolved[func] = res
                continue
            else:
                unresolved[func] = err
                continue

        if setter not in SUPPORTED_SETTERS:
            unresolved[func] = f"setter is {setter}, not a supported copy setter"
            continue

        file_names = names.get(sfile, {})
        file_values = values.get(sfile, {})
        if nm_arr not in file_names or val_arr not in file_values:
            unresolved[func] = (
                f"names/values array not found in {sfile} ({nm_arr}, {val_arr})"
            )
            continue

        libnames, libvals = file_names[nm_arr], file_values[val_arr]
        if len(libnames) != len(libvals):
            unresolved[func] = f"names/values length mismatch ({len(libnames)} vs {len(libvals)})"
            continue

        resolved_vals = []
        bad_vals = []
        for v in libvals:
            ev = safe_eval_expr(v, file_defs.get(sfile))
            if ev is None:
                bad_vals.append(v)
            else:
                resolved_vals.append(ev)

        if bad_vals:
            unresolved[func] = f"non-literal value(s): {bad_vals[:3]}"
            continue

        # Special setter transforms (R-03, V4-02):
        norm_names = []
        if setter == "N_set_ext_params":
            C0 = (math.pi / 3.0) ** (1.0 / 3.0)
            C1 = ((math.pi * math.pi / 36.0) ** (1.0 / 3.0)) / 6.0 - ((math.pi * math.pi / 9.0) ** (1.0 / 3.0)) / 4.0
            N = float(resolved_vals[0])
            gamma, lambda_val = 1.0, 1.0
            if func == "gga_k_absp1":
                gamma = 1.0 - 1.412 / (N ** (1.0 / 3.0))
            elif func == "gga_k_absp2":
                gamma = 1.0 - 1.332 / (N ** (1.0 / 3.0))
            elif func == "gga_k_absp3":
                gamma = 1.0 - 1.513 / (N ** 0.35)
            elif func == "gga_k_absp4":
                gamma = 1.0 / (1.0 + 1.332 / (N ** (1.0 / 3.0)))
                lambda_val = gamma
            elif func == "gga_k_gr":
                gamma = (1.0 - 2.0 / N) * (1.0 - C0 / (N ** (1.0 / 3.0)) + C1 * ((N * N) ** (1.0 / 3.0)))
            elif func == "gga_k_ludena":
                gamma = ((6.0 * math.pi) ** (1.0 / 3.0)) * math.pi * math.pi * (1.0 - 1.0 / (N * N))
            elif func == "gga_k_gp85":
                gamma = ((6.0 * math.pi * math.pi) ** (1.0 / 3.0)) * math.pi * math.pi / 4.0 * (1.0 - 1.0 / N) * (1.0 + 1.0 / N + 6.0 / (N * N))
            elif func == "lda_x_rae":
                dx = 1.0 / ((4.0 * N) ** (1.0 / 3.0))
                dx2 = dx * dx
                alpha = 1.0 - (8.0 / 3.0) * dx + 2.0 * dx2 - (dx2 * dx2) / 3.0
                resolved_vals = [repr(alpha)]
                norm_names = ["param_alpha"]
            if func != "lda_x_rae":
                resolved_vals = [repr(gamma), repr(lambda_val)]
                norm_names = ["param_gamma", "param_lambda"]

        elif setter == "T_set_ext_params":
            # lda_xc_ksdt family
            if func in KSDT_TABLES:
                T = max(float(resolved_vals[0]), 1e-8)
                tab = KSDT_TABLES[func]
                resolved_vals = [repr(T), repr(0.0)]
                norm_names = ["param_T", "param_thetaParam"]
                for i in range(2):
                    for j in range(5):
                        norm_names.append(f"param_b_{i}_{j}")
                        resolved_vals.append(repr(tab["b"][i][j]))
                for i in range(2):
                    for j in range(3):
                        norm_names.append(f"param_c_{i}_{j}")
                        resolved_vals.append(repr(tab["c"][i][j]))
                for i in range(2):
                    for j in range(5):
                        norm_names.append(f"param_d_{i}_{j}")
                        resolved_vals.append(repr(tab["d"][i][j]))
                for i in range(2):
                    for j in range(5):
                        norm_names.append(f"param_e_{i}_{j}")
                        resolved_vals.append(repr(tab["e"][i][j]))
            else:
                T = max(float(resolved_vals[0]), 1e-8)
                resolved_vals = [repr(T)]
                norm_names = ["param_T"]

        elif setter == "pbe_lambda_set_ext_params":
            N, mu, lambda_in = float(resolved_vals[0]), float(resolved_vals[1]), float(resolved_vals[2])
            lambda_val = (1.0 - 1.0 / N) * lambda_in + 1.48 / N
            kappa = lambda_val / (2.0 ** (1.0 / 3.0)) - 1.0
            resolved_vals = [repr(kappa), repr(mu)]
            norm_names = ["param_kappa", "param_mu"]

        elif setter == "s12h_set_ext_params":
            if func in ("hyb_gga_x_cam_s12g", "hyb_gga_x_cam_s12h"):
                A, B, C, D, E, alpha, beta, omega = [float(x) for x in resolved_vals[:8]]
                resolved_vals = [repr(A), repr(B), repr(C), repr(D), repr(E), repr(alpha), repr(beta), repr(omega)]
                norm_names = ["param_A", "param_B", "param_C", "param_D", "param_E", "param_hyb_coeff_1", "param_hyb_coeff_0", "param_hyb_omega_0"]
            else:
                A, B, C, D, E, alpha = [float(x) for x in resolved_vals[:6]]
                bx = 1.0 - alpha
                resolved_vals = [repr(A), repr(B), repr(C), repr(D), repr(E), repr(bx), repr(alpha)]
                norm_names = ["param_A", "param_B", "param_C", "param_D", "param_E", "param_bx", "param_hyb_coeff_0"]

        elif setter == "lcgau_set_ext_params":
            a1, k1, a2, k2, omega = [float(x) for x in resolved_vals]
            w0 = omega
            w2 = omega / math.sqrt(a1)
            w3 = omega / math.sqrt(a2)
            c2 = k1 * math.sqrt(a1)
            c3 = k2 * math.sqrt(a2)
            resolved_vals = [repr(w0), repr(w2), repr(w3), repr(c2), repr(c3)]
            norm_names = ["param_hyb_omega_0", "param_hyb_omega_2", "param_hyb_omega_3", "param_hyb_coeff_2", "param_hyb_coeff_3"]

        elif setter == "mpw91_set_ext_params":
            bt, alpha, expo = [float(x) for x in resolved_vals]
            X2S = 0.1282782438530421943003058721616428784381
            X_FACTOR_C = 0.9305257363491000250020102180716672510262
            beta = 5.0 * ((36.0 * math.pi) ** (-5.0 / 3.0))
            a = 6.0 * bt / X2S
            b = 1.0 / X2S
            c = bt / (X_FACTOR_C * X2S * X2S)
            d = -(bt - beta) / (X_FACTOR_C * X2S * X2S)
            f = 1.0e-6 / (X_FACTOR_C * (X2S ** expo))
            resolved_vals = [repr(a), repr(b), repr(c), repr(d), repr(f), repr(alpha), repr(expo)]
            norm_names = ["param_a", "param_b", "param_c", "param_d", "param_f", "param_alpha", "param_expo"]

        elif setter in ("lspbe_set_ext_params", "lsrpbe_set_ext_params"):
            # mu += alpha * (1.0 + kappa)
            val_map = dict(zip(libnames, [float(x) for x in resolved_vals]))
            val_map["_mu"] += val_map["_alpha"] * (1.0 + val_map["_kappa"])
            resolved_vals = [repr(val_map[n]) for n in libnames]

        if not norm_names:
            for n in libnames:
                if (func, n) in PARAM_ALIASES:
                    norm_names.append(PARAM_ALIASES[(func, n)])
                    continue
                if (base, n) in PARAM_ALIASES:
                    norm_names.append(PARAM_ALIASES[(base, n)])
                    continue
                if n in ("_omega", "short_range_omega") and "param_hyb_omega_0" in kp:
                    norm_names.append("param_hyb_omega_0")
                    continue
                if n in ("_exx", "_ax", "exx") and "param_hyb_coeff_0" in kp:
                    norm_names.append("param_hyb_coeff_0")
                    continue
                name = re.sub(r"\[(\d+)\]", r"_\1", n)
                pname = f"param{name}"
                if pname not in kp:
                    alt = "param" + re.sub(r"([a-zA-Z]+)(\d+)", r"\1_\2", n)
                    if alt in kp:
                        pname = alt
                    elif "param" + re.sub(r"([a-zA-Z]+)(\d)(\d)", r"\1_\2_\3", n) in kp:
                        pname = "param" + re.sub(r"([a-zA-Z]+)(\d)(\d)", r"\1_\2_\3", n)
                    elif "param_" + n.lstrip("_") in kp:
                        pname = "param_" + n.lstrip("_")
                norm_names.append(pname)

        # Filter to parameters present in kernel (dropping unused helper parameters like exx in pure kernel)
        mapping = {}
        for n, v in zip(norm_names, resolved_vals):
            if n in kp:
                mapping[n] = v

        # s12g needs bx default 1.0 from init
        if func == "gga_x_s12g" and "param_bx" in kp and "param_bx" not in mapping:
            mapping["param_bx"] = "1.0"

        if set(mapping) != set(kp):
            only_k = sorted(set(kp) - set(mapping))
            only_l = sorted(set(norm_names) - set(kp))
            unresolved[func] = (
                f"param set mismatch; kernel-only={only_k} libxc-only={only_l}"
            )
            continue

        # Runtime ext_params wiring.
        #
        # libxc's `copy_params` (util.c:94) writes `ext_params[ii]` into slot
        # `ii` of the functional's C params struct, so the struct field order
        # IS the ext_params order -- that identity is what makes
        # `set_ext_params_cpy` correct. The kernel's *argument* order is a
        # different thing: `from_maple.py` takes it from the maple2c body, and
        # for 160 of 276 functionals it is a permutation of the ext_params
        # order (`gga_c_pbe` is `[gamma, BB, beta]` against libxc's
        # `[_beta, _gamma, _B]`). Feeding ext_params to a kernel positionally
        # would therefore silently swap constants.
        #
        # `norm_names[i]` is already the kernel parameter that libxc ext_param
        # `libnames[i]` feeds -- that mapping is built by name above, with an
        # explicit alias table, and the block below this refuses the functional
        # outright if it does not cover every kernel parameter. So the
        # permutation is just the index of each `norm_names` entry in `kp`,
        # with `None` for an ext_param the kernel does not consume (e.g. `_exx`
        # on a pure exchange kernel, dropped by the `mapping` filter above).
        #
        # If the two lists cannot be put in correspondence at all, emit `None`
        # rather than a guess: `gen_eval.py` turns that into a dispatch that
        # rejects runtime ext_params and keeps using the compiled-in defaults.
        if len(norm_names) == len(libnames):
            ext_to_kernel = [kp.index(n) if n in kp else None for n in norm_names]
            ext_names = list(libnames)
        else:
            ext_to_kernel = None
            ext_names = None

        # Gate it on the metadata the eval layer will actually hand us.
        ext_names, ext_to_kernel, why = validate_ext_wiring(
            func, kp, [mapping[p] for p in kp], ext_names, ext_to_kernel, meta_ext)
        if why:
            ext_unwired[func] = why

        resolved[func] = {
            "family": fam,
            "base_kernel": base,
            "params": kp,
            "values": [mapping[p] for p in kp],
            "ext_names": ext_names,
            "ext_to_kernel": ext_to_kernel,
        }

    n_ext_wired = sum(1 for v in resolved.values() if v.get("ext_to_kernel"))
    print(f"runtime ext_params wired : {n_ext_wired}")
    if ext_unwired:
        print(f"runtime ext_params REFUSED for {len(ext_unwired)} "
              f"(kept on compiled-in defaults):")
        groups: dict[str, list[str]] = {}
        for f, why in ext_unwired.items():
            groups.setdefault(why.split(";")[0].split("=")[0].strip(), []).append(f)
        for key, fs in sorted(groups.items(), key=lambda kv: -len(kv[1])):
            print(f"  {len(fs):3d}  {key}")
            print(f"       e.g. {', '.join(sorted(fs)[:4])}")

    n_paramless = sum(1 for v in resolved.values() if not v["params"])
    n_wired = len(resolved) - n_paramless
    print(f"resolved   : {len(resolved)}  ({n_paramless} paramless, {n_wired} with defaults)")
    print(f"unresolved : {len(unresolved)}")
    if unresolved:
        reasons = {}
        for f, why in unresolved.items():
            key = why.split(";")[0].split("(")[0].strip()
            reasons.setdefault(key, []).append(f)
        print("\nunresolved by reason:")
        for key, fs in sorted(reasons.items(), key=lambda kv: -len(kv[1])):
            print(f"  {len(fs):3d}  {key}")
            print(f"       e.g. {', '.join(fs[:4])}")

    if args.json:
        Path(args.json).write_text(json.dumps(
            {"resolved": resolved, "unresolved": unresolved}, indent=1))
        print(f"\nwrote {args.json}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
