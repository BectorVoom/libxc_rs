//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1996/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1996<F: Float>(t100646: F, t100659: F, t100689: F, t100718: F, t100734: F, t100747: F, t100759: F, t100772: F, t100780: F, t1877: F, t2057: F, t24191: F, t24339: F, t2522: F, t25934: F, t25945: F, t26744: F, t26756: F, t28764: F, t28789: F, t28792: F, t4314: F, t5966: F, t7110: F, t7114: F, t84800: F) -> F {
    let t102087 = t1877 * t7110 * t5966 / F::new(2.0) - t1877 * t7114 * t100646 / F::new(2.0) - F::new(3.0) * t24191 * t100780 - t1877 * t24339 * t28792 - t1877 * t7114 * t100772 / F::new(2.0) + F::new(3.0) * t4314 * t2057 * t100759 + F::new(3.0) / F::new(2.0) * t2522 * t2057 * t100747 + F::new(3.0) * t4314 * t7110 * t28764 + F::new(2.0) * t26756 * t100689 + F::new(2.0) * t26756 * t100659 + t1877 * t84800 * t28789 - t1877 * t7114 * t100734 / F::new(2.0) + F::new(3.0) * t2522 * t2057 * t100718 - t1877 * t26744 * t25945 - t1877 * t26744 * t25934;
    t102087
}
