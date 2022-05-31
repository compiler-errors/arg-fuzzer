
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4520(_: S1, _: S4) {}
        
        fn test4520() { foo4520(S5, S5, S5, S6, S1, S3); }
    