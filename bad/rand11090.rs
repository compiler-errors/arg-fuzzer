
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11090(_: S1, _: S2, _: S3, _: S5, _: S6) {}
        
        fn test11090() { foo11090(S5, S0, S1, S5, S3, S0); }
    