//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 935/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk935<F: Float>(t11002: F, t3130: F, t1025: F, t1041: F, t1046: F, t10932: F, t10937: F, t10944: F, t10949: F, t10952: F, t10957: F, t10962: F, t10965: F, t10972: F, t10982: F, t10985: F, t10988: F, t10994: F, t10998: F, t2960: F, t3043: F, t3048: F, t3057: F, t3064: F, t3073: F, t3117: F, t3134: F, t3143: F, t3148: F, t3153: F, t973: F) -> F {
    let t11003 = t3130 * t11002;
    let t11005 = -t973 * t10932 / F::cast_from(36.0_f64) - t10937 * t3073 / F::cast_from(144.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t3117 * t3064 + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t973 * t10944 + t10949 * t3134 / F::cast_from(512.0_f64) - t10952 * t3043 / F::cast_from(1024.0_f64) + F::cast_from(19.0_f64) / F::cast_from(864.0_f64) * t10957 * t1046 + t10962 * t1025 / F::cast_from(1024.0_f64) + t10965 * t1046 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t1041 * t10972 - t3048 * t3057 / F::cast_from(288.0_f64) - t2960 * t3143 / F::cast_from(36.0_f64) - t2960 * t3148 / F::cast_from(27.0_f64) + t10982 / F::cast_from(288.0_f64) + t10985 / F::cast_from(216.0_f64) + t973 * t10988 / F::cast_from(288.0_f64) + t2960 * t3153 / F::cast_from(18.0_f64) - t10994 / F::cast_from(144.0_f64) + t973 * t10998 / F::cast_from(48.0_f64) + t11003 / F::cast_from(768.0_f64);
    t11005
}
