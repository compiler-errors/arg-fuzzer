
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8076(_: S4, _: S8, _: S3, _: S5, _: S2, _: S1, _: S7) {}
        
        fn test8076() { foo8076(S4, S0, S5, S7, S3); }
    