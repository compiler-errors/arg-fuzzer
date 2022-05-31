
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo163(_: S5, _: S6, _: S5, _: S4) {}
        
        fn test163() { foo163(S6, S3, S0, S5, S6, S3, S3); }
    