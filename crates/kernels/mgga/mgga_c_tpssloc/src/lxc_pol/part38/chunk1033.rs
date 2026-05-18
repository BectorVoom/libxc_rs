//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1033/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1033<F: Float>(t103: F, t584: F, t16: F, t4063: F, t100: F, t12771: F, t12774: F, t12775: F, t12778: F, t12781: F, t12784: F, t12792: F, t12795: F, t12796: F, t12799: F, t1445: F, t1447: F, t2336: F, t2351: F, t2355: F, t4050: F, t4054: F, t657: F, t92: F) -> F {
    let t12802 = t103 * t584;
    let t12805 = t4063 * t16;
    let t12808 = F::new(200.0) / F::new(27.0) * t2336 * t1445 - F::new(100.0) / F::new(27.0) * t657 * t4050 - F::new(50.0) / F::new(9.0) * t657 * t4054 - F::new(10.0) / F::new(27.0) * t92 * t12771 + F::new(20.0) / F::new(9.0) * t12774 * t12775 + F::new(10.0) / F::new(9.0) * t92 * t12778 + F::new(5.0) / F::new(3.0) * t92 * t12781 - F::new(5.0) * t92 * t12784 - F::new(50.0) / F::new(27.0) * t1447 * t2351 - F::new(25.0) / F::new(9.0) * t1447 * t2355 - F::new(10.0) / F::new(27.0) * t100 * t12792 - F::new(20.0) / F::new(9.0) * t12795 * t12796 + F::new(10.0) / F::new(9.0) * t100 * t12799 - F::new(5.0) / F::new(3.0) * t100 * t12802 + F::new(5.0) * t100 * t12805;
    t12808
}
