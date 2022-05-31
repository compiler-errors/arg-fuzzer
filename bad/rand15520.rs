
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15520(_: S2, _: S3, _: S5, _: S7, _: S8) {}
        
        fn test15520() { foo15520(S1, S0, S4, S5, S0, S3); }
    