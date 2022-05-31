
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3926(_: S4, _: S5, _: S8) {}
        
        fn test3926() { foo3926(S7, S6, S5, S1, S7); }
    