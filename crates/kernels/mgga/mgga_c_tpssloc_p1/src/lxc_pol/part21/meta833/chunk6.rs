//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2947/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2947<F: Float>(t4509: F, t5842: F, t17686: F, t42841: F, t17783: F, t2960: F, t13779: F, t17167: F, t2986: F, t10235: F, t10237: F, t10241: F, t10245: F, t10263: F, t17804: F, t17817: F, t17863: F, t42846: F, t4518: F, t48281: F, t5818: F, t5825: F, t59659: F) -> F {
    let t61365 = t4509 * t5842;
    let t61375 = t42841 * t17686;
    let t61383 = t2960 * t17783;
    let t61387 = t2986 * t13779 * t17167;
    let t61389 = F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t10241 * t17817 - F::cast_from(0.27777777777777777777e-3_f64) * t2986 * t17804 * t10245 - F::cast_from(0.37037037037037037036e-3_f64) * t2986 * t61365 * t10237 - F::cast_from(0.66666666666666666664e-2_f64) * t2986 * t4518 * t59659 - F::cast_from(0.37037037037037037036e-3_f64) * t2986 * t42846 * t17863 + F::cast_from(0.44444444444444444442e-2_f64) * t2986 * t10235 * t61375 - F::cast_from(0.54320987654320987651e-2_f64) * t10263 * t5825 + F::cast_from(0.36213991769547325102e-2_f64) * t10263 * t5818 - F::cast_from(0.6584362139917695473e-3_f64) * t61383 - F::cast_from(0.24691358024691358024e-3_f64) * t48281 + F::cast_from(0.11111111111111111111e-2_f64) * t61387;
    t61389
}
