
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10744(_: S1, _: S8) {}
        
        fn test10744() { foo10744(S6, S1, S4, S2); }
    