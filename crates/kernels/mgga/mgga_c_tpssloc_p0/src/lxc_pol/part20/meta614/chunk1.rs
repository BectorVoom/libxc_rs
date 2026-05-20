//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2205/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2205<F: Float>(t100: F, t9398: F, t2341: F, t657: F, t12774: F, t12775: F, t12778: F, t12795: F, t1447: F, t2219: F, t2248: F, t2336: F, t2342: F, t2350: F, t2354: F, t30171: F, t30307: F, t4049: F, t4050: F, t4054: F, t45697: F, t659: F, t662: F, t92: F, t9212: F, t9393: F, t9404: F) -> F {
    let t45707 = t100 * t9398;
    let t45717 = t657 * t2341;
    let t45731 = -F::new(10.0) * t12774 * t9212 * t659 + F::new(10.0) * t12795 * t9212 * t662 - F::new(10.0) / F::new(9.0) * t45697 * t30171 * t2248 - F::new(10.0) / F::new(9.0) * t45697 * t2219 * t2342 + F::new(10.0) / F::new(3.0) * t12774 * t2219 * t2248 - F::new(10.0) / F::new(9.0) * t45707 * t30307 * t2354 + F::new(10.0) / F::new(9.0) * t45707 * t2219 * t2350 - F::new(10.0) / F::new(3.0) * t12795 * t2219 * t2354 - F::new(100.0) / F::new(9.0) * t45717 * t12775 + F::new(400.0) / F::new(27.0) * t2336 * t4050 + F::new(200.0) / F::new(9.0) * t2336 * t4054 - F::new(50.0) / F::new(9.0) * t657 * t12778 + F::new(10.0) / F::new(9.0) * t92 * t4049 * t9393 - F::new(50.0) / F::new(9.0) * t1447 * t9404;
    t45731
}
