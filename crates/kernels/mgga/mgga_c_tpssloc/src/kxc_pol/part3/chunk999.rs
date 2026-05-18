//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 999/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk999<F: Float>(t157: F, t9929: F, t4196: F, t9726: F, t10143: F, t1530: F, t2430: F, t4205: F, t1409: F, t750: F, t607: F, t4194: F) -> (F, F, F, F, F) {
    let t12908 = t9929 * t157;
    let t12910 = F::new(24.0) * t12908 * t4196;
    let t12914 = F::new(2.0) * t9726;
    let t12915 = t1530 * t10143;
    let t12922 = F::new(8.0) * t4205 * t2430;
    let t12923 = t750 * t1409;
    let t12924 = t12923 * t607;
    let t12926 = F::new(24.0) * t4194 * t12924;
    (t12910, t12914, t12915, t12922, t12926)
}
