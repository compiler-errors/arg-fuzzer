
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4290(_: S1, _: S3, _: S7, _: S5, _: S5) {}
        
        fn test4290() { foo4290(S6, S2, S6, S3, S5, S6, S3); }
    