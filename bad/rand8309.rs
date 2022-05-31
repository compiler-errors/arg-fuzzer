
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8309(_: S1, _: S2, _: S3, _: S4, _: S5, _: S6, _: S7) {}
        
        fn test8309() { foo8309(S1, S7, S7, S6, S0); }
    