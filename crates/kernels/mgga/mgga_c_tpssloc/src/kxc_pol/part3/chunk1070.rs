//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1070/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1070<F: Float>(t10189: F, t1597: F, t2990: F, t2986: F, t2987: F, t4540: F, t10245: F, t4531: F, t10241: F, t4514: F, t2989: F, t3966: F) -> (F, F, F, F, F) {
    let t13847 = t10189 * t1597;
    let t13848 = t13847 * t2990;
    let t13850 = F::new(0.18518518518518518518e-3) * t2986 * t13848;
    let t13851 = t2987 * t4540;
    let t13852 = t13851 * t2990;
    let t13855 = t4531 * t10245;
    let t13858 = t10241 * t4514;
    let t13861 = t2989 * t3966;
    (t13850, t13852, t13855, t13858, t13861)
}
