//! GGA_K_OL1 kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_ol1.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_ol1_kxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = sigma[ip] * t25;
        let t27 = rho[ip] * rho[ip];
        let t29 = 1.0 / t22 / t27;
        let t32 = f64::sqrt(sigma[ip]);
        let t33 = t25 * t32;
        let t35 = 1.0 / t21 / rho[ip];
        let t39 = M_CBRT6;
        let t41 = M_PI * M_PI;
        let t42 = pow_1_3(t41);
        let t43 = t42 * t42;
        let t44 = 1.0 / t43;
        let t47 = 1.0 + 5.0 / 9.0 * (t26 * t29 / 72.0 + 0.677e-2 * t33 * t35) * t39 * t44;
        let t51 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t20 * t22 * t47);
        let tzk0 = 2.0 * t51;
        zk[ip] += tzk0;
        let t52 = 1.0 / t21;
        let t57 = t7 * t20;
        let t58 = t27 * rho[ip];
        let t60 = 1.0 / t22 / t58;
        let t64 = 1.0 / t21 / t27;
        let t67 = -t26 * t60 / 27.0 - 0.90266666666666666666e-2 * t33 * t64;
        let t69 = t39 * t44;
        let t74 = piecewise3(t2, 0.0, t7 * t20 * t52 * t47 / 10.0 + t57 * t22 * t67 * t69 / 12.0);
        let tvrho0 = 2.0 * rho[ip] * t74 + 2.0 * t51;
        vrho[ip] += tvrho0;
        let t80 = t25 / t32;
        let t83 = t25 * t29 / 72.0 + 0.3385e-2 * t80 * t35;
        let t88 = piecewise3(t2, 0.0, t57 * t22 * t83 * t69 / 12.0);
        let tvsigma0 = 2.0 * rho[ip] * t88;
        vsigma[ip] += tvsigma0;
        let t99 = t27 * t27;
        let t101 = 1.0 / t22 / t99;
        let t105 = 1.0 / t21 / t58;
        let t108 = 11.0 / 81.0 * t26 * t101 + 0.21062222222222222222e-1 * t33 * t105;
        let t114 = piecewise3(t2, 0.0, -t7 * t20 * t35 * t47 / 30.0 + t57 * t52 * t67 * t69 / 9.0 + t57 * t22 * t108 * t69 / 12.0);
        let tv2rho20 = 2.0 * rho[ip] * t114 + 4.0 * t74;
        v2rho2[ip] += tv2rho20;
        let t125 = -t25 * t60 / 27.0 - 0.45133333333333333333e-2 * t80 * t64;
        let t131 = piecewise3(t2, 0.0, t57 * t52 * t83 * t69 / 18.0 + t57 * t22 * t125 * t69 / 12.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t131 + 2.0 * t88;
        v2rhosigma[ip] += tv2rhosigma0;
        let t134 = t4 * t20;
        let t136 = t134 / t22;
        let t140 = t25 / t32 / sigma[ip] * t69;
        let t143 = piecewise3(t2, 0.0, -0.64895402177010868827e-3 * t136 * t140);
        let tv2sigma20 = 2.0 * rho[ip] * t143;
        v2sigma2[ip] += tv2sigma20;
        let t158 = t99 * rho[ip];
        let t160 = 1.0 / t22 / t158;
        let t164 = 1.0 / t21 / t99;
        let t167 = -154.0 / 243.0 * t26 * t160 - 0.70207407407407407407e-1 * t33 * t164;
        let t173 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t20 * t64 * t47 - t57 * t35 * t67 * t69 / 18.0 + t57 * t52 * t108 * t69 / 6.0 + t57 * t22 * t167 * t69 / 12.0);
        let tv3rho30 = 2.0 * rho[ip] * t173 + 6.0 * t114;
        v3rho3[ip] += tv3rho30;
        let t189 = 11.0 / 81.0 * t25 * t101 + 0.10531111111111111111e-1 * t80 * t105;
        let t195 = piecewise3(t2, 0.0, -t57 * t35 * t83 * t69 / 54.0 + t57 * t52 * t125 * t69 / 9.0 + t57 * t22 * t189 * t69 / 12.0);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t195 + 4.0 * t131;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t200 = t134 / t22 / rho[ip];
        let t203 = piecewise3(t2, 0.0, 0.43263601451340579218e-3 * t200 * t140);
        let tv3rhosigma20 = 2.0 * rho[ip] * t203 + 2.0 * t143;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t206 = sigma[ip] * sigma[ip];
        let t210 = t25 / t32 / t206 * t69;
        let t213 = piecewise3(t2, 0.0, 0.9734310326551630324e-3 * t136 * t210);
        let tv3sigma30 = 2.0 * rho[ip] * t213;
        v3sigma3[ip] += tv3sigma30;
    }
}
