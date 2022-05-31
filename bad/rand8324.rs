
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8324(_: S5, _: S2) {}
        
        fn test8324() { foo8324(S1, S4, S5, S7, S0); }
    