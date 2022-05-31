
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14277(_: S2, _: S4, _: S8) {}
        
        fn test14277() { foo14277(S2, S3, S4, S5, S6); }
    