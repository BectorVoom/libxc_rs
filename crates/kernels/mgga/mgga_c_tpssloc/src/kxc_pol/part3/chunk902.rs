//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 902/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk902<F: Float>(t16: F, t4053: F, t1449: F, t2350: F, t9398: F, t100: F, t2349: F, t2219: F, t662: F, t2354: F, t4059: F, t103: F, t584: F, t4063: F, t12771: F, t12774: F, t12775: F, t12778: F, t12781: F, t1445: F, t1447: F, t2336: F, t2351: F, t2355: F, t4050: F, t4054: F, t657: F, t92: F) -> (F,) {
    let t12784 = t4053 * t16;
    let t12792 = t9398 * t1449 * t2350;
    let t12795 = t100 * t2349;
    let t12796 = t2219 * t662;
    let t12799 = t4059 * t2354;
    let t12802 = t103 * t584;
    let t12805 = t4063 * t16;
    let t12808 = 200.0 / 27.0 * t2336 * t1445 - 100.0 / 27.0 * t657 * t4050 - 50.0 / 9.0 * t657 * t4054 - 10.0 / 27.0 * t92 * t12771 + 20.0 / 9.0 * t12774 * t12775 + 10.0 / 9.0 * t92 * t12778 + 5.0 / 3.0 * t92 * t12781 - 5.0 * t92 * t12784 - 50.0 / 27.0 * t1447 * t2351 - 25.0 / 9.0 * t1447 * t2355 - 10.0 / 27.0 * t100 * t12792 - 20.0 / 9.0 * t12795 * t12796 + 10.0 / 9.0 * t100 * t12799 - 5.0 / 3.0 * t100 * t12802 + 5.0 * t100 * t12805;
    (t12808,)
}
