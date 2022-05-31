
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11806(_: S5, _: S2, _: S3, _: S7) {}
        
        fn test11806() { foo11806(S4, S6, S4, S0, S7, S5, S6); }
    