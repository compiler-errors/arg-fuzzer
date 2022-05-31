
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16370(_: S0, _: S5, _: S2, _: S5, _: S7) {}
        
        fn test16370() { foo16370(S1, S2, S3, S4, S5, S8); }
    