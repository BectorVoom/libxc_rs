//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 684/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk684<F: Float>(t23086: F, t6605: F, t2684: F, t815: F, t23043: F, t23044: F, t23049: F, t23051: F, t23054: F, t23057: F, t23059: F, t23063: F, t23067: F, t23071: F, t23073: F, t23081: F, t23084: F) -> (F, F, F) {
    let t23087 = t6605 * t23086;
    let t23089 = t815 * t2684;
    let t23090 = t6605 * t23089;
    let t23092 = t23043 - t23044 / 1536.0 + t23049 / 768.0 - t23051 / 1536.0 - t23054 / 768.0 + t23057 / 16.0 - t23059 / 48.0 + 0.16956557559538964159e-1 * t23063 - 0.12111826828242117256e-2 * t23067 + t23071 + 0.40372756094140390854e-3 * t23073 + 0.84782787797694820792e-2 * t23081 + 0.28260929265898273598e-2 * t23084 - 0.20186378047070195427e-3 * t23087 - 0.20186378047070195427e-3 * t23090;
    (t23087, t23090, t23092)
}
