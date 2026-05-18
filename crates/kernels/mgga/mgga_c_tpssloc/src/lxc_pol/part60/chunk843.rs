//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 843/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk843<F: Float>(t24776: F, t29776: F, t6256: F, t7376: F, t7375: F, t27516: F, t8066: F, t1716: F, t8077: F, t1729: F, t2152: F, t24589: F, t24812: F, t27406: F, t27507: F, t27572: F, t27728: F, t27737: F, t29750: F, t29754: F, t29759: F, t29763: F, t29773: F, t470: F, t6168: F, t7283: F, t7373: F, t7999: F, t8067: F, t8074: F, t8078: F, t8085: F) -> F {
    let t29777 = t24776 * t29776;
    let t29781 = t6256 * t7376;
    let t29782 = t7375 * t29781;
    let t29787 = t27516 * t8066;
    let t29790 = t1716 * t8077;
    let t29793 = F::new(0.16449340668482264365e-1) * t24812 * t29750 - F::new(0.82246703342411321825e-2) * t24812 * t29754 - F::new(0.14621636149762012769e-1) * t27572 - F::new(0.27415567780803773942e-2) * t7283 * t29759 - F::new(0.54831135561607547884e-2) * t7283 * t29763 + F::new(0.14621636149762012769e-1) * t27406 * t8067 - F::new(0.43864908449286038306e-1) * t7999 * t8078 + t6168 * t2152 + F::new(2.0) * t1729 * t8085 + t470 * t29773 - F::new(0.54831135561607547884e-2) * t27728 + F::new(0.36554090374405031923e-2) * t7283 * t29777 + F::new(0.54831135561607547884e-2) * t27737 + F::new(0.16449340668482264365e-1) * t7373 * t29782 - F::new(0.43864908449286038306e-1) * t27507 * t8074 + F::new(0.54831135561607547884e-2) * t24589 * t29787 - F::new(0.16449340668482264365e-1) * t7283 * t29790;
    t29793
}
