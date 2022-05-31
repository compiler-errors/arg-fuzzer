
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10926(_: S6, _: S5) {}
        
        fn test10926() { foo10926(S4, S3, S4, S0, S6, S2); }
    